use super::*;

/// Keeps the opt-in exception on one exact HWND/PID; selector mismatch or duplicate rows fail closed.
pub(super) fn surface_windows(
    adapter: &dyn PlatformAdapter,
    app_name: Option<&str>,
    window_id: Option<&str>,
    surface: crate::SnapshotSurface,
    target: Option<crate::CloakedWindowTarget>,
    deadline: crate::Deadline,
) -> Result<Vec<WindowInfo>, AppError> {
    if let Some(target) = target {
        if surface != crate::SnapshotSurface::Window {
            return Err(AppError::invalid_input(
                "A cloaked-window target supports the window surface only",
            ));
        }
        let target_id = target.window_id();
        if window_id.is_some_and(|requested| requested != target_id) {
            return Err(crate::AdapterError::new(
                crate::ErrorCode::WindowNotFound,
                "The requested window selector does not match --cloaked-window",
            )
            .unsent()
            .into());
        }
        let windows = adapter.list_windows_with_filter(
            &WindowFilter {
                focused_only: false,
                app: app_name.map(str::to_string),
                cloaked_window: Some(target),
            },
            deadline,
        )?;
        let candidates: Vec<_> = windows
            .into_iter()
            .filter(|window| {
                window.id == target_id
                    && window.pid == target.pid
                    && app_name.is_none_or(|app| crate::app_name_matches(&window.app, app))
            })
            .collect();
        return crate::system::window_lookup::select_window(
            candidates,
            crate::AdapterError::new(
                crate::ErrorCode::WindowNotFound,
                "The permitted window is no longer an eligible top-level window",
            ),
            "Multiple windows matched the exact permitted window identity",
        )
        .map(|window| vec![window]);
    }
    use crate::SnapshotSurface::{Alert, Popover, Sheet};
    if window_id.is_none() && matches!(surface, Sheet | Popover | Alert) {
        let mut windows = match app_name {
            Some(_) => super::windows_for_app(adapter, app_name, deadline)?,
            None => super::frontmost_process_windows(adapter, surface, deadline)?,
        };
        windows.retain(|window| window.state.accessible);
        let windows = crate::system::window_lookup::surface_owner_order(windows);
        if !windows.is_empty() {
            return Ok(windows);
        }
    }
    Ok(vec![super::resolve_window_for_surface(
        adapter, app_name, window_id, surface, deadline,
    )?])
}

#[cfg(test)]
#[path = "window_target_tests.rs"]
mod tests;
