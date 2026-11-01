use agent_desktop_core::{AdapterError, Deadline, ErrorCode, WindowFilter, WindowInfo};

use super::window_enum::{enumerate_top_level, read_cloak_raw};
use super::window_identity::{WindowIdentityEvidence, live_window_title};
use super::window_ops::{is_foreground_window, parse_handle, passes_filter, window_state};
use crate::system::process_identity;

/// Resolves a live window by `WindowInfo.id`, corroborating pid and process
/// generation against the handle's current owner (stored-evidence rule).
pub(crate) fn resolve_window_strict(
    expected: &WindowInfo,
    deadline: Deadline,
) -> Result<WindowInfo, AdapterError> {
    resolve_window_strict_with_filter(expected, &WindowFilter::default(), deadline)
}

pub(crate) fn resolve_window_strict_with_filter(
    expected: &WindowInfo,
    filter: &WindowFilter,
    deadline: Deadline,
) -> Result<WindowInfo, AdapterError> {
    deadline.ensure_remaining()?;
    let handle = parse_handle(&expected.id);
    if handle.is_null() {
        return Err(AdapterError::new(
            ErrorCode::InvalidArgs,
            "window id is not a Windows HWND-shaped id",
        ));
    }
    if !window_exists(handle) {
        return Err(AdapterError::new(
            ErrorCode::WindowNotFound,
            "window handle is no longer a live top-level window",
        ));
    }
    let Some(evidence) = WindowIdentityEvidence::from_info(handle, expected) else {
        return Err(AdapterError::new(
            ErrorCode::StaleRef,
            "headed focus requires a process-instance token on the stored window",
        ));
    };
    if let Some(target) = filter.cloaked_window {
        if target.window_id() != expected.id || target.pid != expected.pid {
            return Err(AdapterError::new(
                ErrorCode::StaleRef,
                "window ref does not match the one-command cloaked-window permission",
            ));
        }
        let current_instance = process_identity::token_for_pid(expected.pid)?;
        if current_instance.as_deref() != expected.process_instance.as_deref() {
            return Err(AdapterError::new(
                ErrorCode::StaleRef,
                "window process generation changed after the ref was issued",
            ));
        }
        if !evidence.owns_handle_now()? {
            return Err(AdapterError::new(
                ErrorCode::StaleRef,
                "window handle no longer belongs to the process generation in the ref",
            ));
        }
    } else {
        evidence.verify_stored()?;
    }
    live_window_info(handle, expected, filter)
}

fn live_window_info(
    handle: super::window_enum::WindowHandle,
    expected: &WindowInfo,
    filter: &WindowFilter,
) -> Result<WindowInfo, AdapterError> {
    let mut found = None;
    enumerate_top_level(|window| {
        if window.handle != handle {
            return true;
        }
        found = Some(window);
        false
    })?;
    let window = found.ok_or_else(|| {
        AdapterError::new(
            ErrorCode::WindowNotFound,
            "window handle is no longer present in the top-level inventory",
        )
    })?;
    if let Some(target) = filter.cloaked_window {
        if target.window_id() != expected.id || target.pid != expected.pid {
            return Err(AdapterError::new(
                ErrorCode::StaleRef,
                "window does not match the one-command cloaked-window permission",
            ));
        }
        if filter
            .app
            .as_deref()
            .is_some_and(|app| !agent_desktop_core::app_name_matches(&expected.app, app))
        {
            return Err(AdapterError::new(
                ErrorCode::WindowNotFound,
                "window does not match the application selector",
            ));
        }
        let evidence = super::cloaked_target::TargetWindowEvidence {
            hwnd: handle as usize as u64,
            pid: expected.pid.get(),
            process_instance: process_identity::token_for_pid(expected.pid)?,
            visible: window.visible,
            nonzero: !window.is_zero_sized(),
            tool: window.tool,
            cloak_raw: read_cloak_raw(handle),
        };
        if !super::cloaked_target::matches_identity(
            target,
            &evidence,
            expected.process_instance.as_deref(),
        ) {
            return Err(AdapterError::new(
                ErrorCode::StaleRef,
                "window process generation changed while resolving the cloaked-window ref",
            ));
        }
        if !super::cloaked_target::eligible(target, &evidence, expected.process_instance.as_deref())
        {
            return Err(AdapterError::new(
                ErrorCode::WindowNotFound,
                "window no longer satisfies the cloaked-window visibility and identity gates",
            ));
        }
    } else if !passes_filter(&window) && !window.iconic {
        return Err(AdapterError::new(
            ErrorCode::WindowNotFound,
            "window is no longer an agent-visible top-level window",
        ));
    }
    let focused = is_foreground_window(handle);
    Ok(WindowInfo {
        id: expected.id.clone(),
        title: live_window_title(handle).unwrap_or_else(|| expected.title.clone()),
        app: expected.app.clone(),
        pid: expected.pid,
        process_instance: expected.process_instance.clone(),
        bounds: Some(window.rect),
        state: window_state(focused, window.iconic, window.visible),
    })
}

#[cfg(target_os = "windows")]
fn window_exists(handle: super::window_enum::WindowHandle) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::IsWindow;
    unsafe { IsWindow(handle) != 0 }
}

#[cfg(not(target_os = "windows"))]
fn window_exists(_handle: super::window_enum::WindowHandle) -> bool {
    false
}

#[cfg(test)]
#[path = "window_resolve_tests.rs"]
mod tests;
