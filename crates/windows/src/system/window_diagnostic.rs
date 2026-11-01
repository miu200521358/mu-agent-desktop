use agent_desktop_core::{
    AdapterError, Deadline, ErrorCode, VirtualDesktopDiagnostic, WindowDiagnosticIdentity,
    WindowDiagnosticQuery, WindowDiagnosticRecord, WindowDiagnosticReport, WindowDiagnosticState,
};

use super::window::window_enum::{enumerate_top_level, read_cloak_raw};

const CLSID_VIRTUAL_DESKTOP_MANAGER: windows::core::GUID =
    windows::core::GUID::from_u128(0xaa509086_5ca9_4c25_8f95_589d3c07b48a);

/// Reports matching top-level windows without creating refs; failed native reads remain unknown.
pub(crate) fn diagnose(
    query: &WindowDiagnosticQuery,
    deadline: Deadline,
) -> Result<WindowDiagnosticReport, AdapterError> {
    if query.pid.get() == 0 {
        return Err(AdapterError::new(
            ErrorCode::InvalidArgs,
            "window-diagnostic requires a positive pid",
        ));
    }
    if let Some(window_id) = query.window_id.as_deref() {
        agent_desktop_core::CloakedWindowTarget::parse(window_id, query.pid.get())?;
    }
    deadline.ensure_remaining()?;
    let virtual_desktops = VirtualDesktopProbe::new();
    let records = std::cell::RefCell::new(Vec::new());
    let failure = std::cell::RefCell::new(None);
    enumerate_top_level(|window| {
        if let Err(error) = deadline.ensure_remaining() {
            *failure.borrow_mut() = Some(error);
            return false;
        }
        let Some(pid) = super::window::window_identity::live_window_owner(window.handle) else {
            return true;
        };
        if pid != query.pid {
            return true;
        }
        let id = format!("w-{}", window.handle as usize);
        let app = super::process_identity::process_image_name(pid).unwrap_or_default();
        if !super::window::diagnostic::matches_query(query, &id, &app, pid.get()) {
            return true;
        }
        let process_instance = super::process_identity::token_for_pid(pid).ok().flatten();
        if super::window::window_identity::live_window_owner(window.handle) != Some(pid) {
            return true;
        }
        let cloak = super::window::diagnostic::cloak_diagnostic(read_cloak_raw(window.handle));
        let virtual_desktop = virtual_desktops.read(window.handle);
        records.borrow_mut().push(WindowDiagnosticRecord {
            identity: WindowDiagnosticIdentity {
                id,
                app,
                pid,
                process_instance,
            },
            state: WindowDiagnosticState {
                visible: window.visible,
                iconic: window.iconic,
                tool: window.tool,
                bounds: Some(window.rect),
            },
            cloak,
            virtual_desktop,
            normal_inventory: super::window::diagnostic::normal_inventory_decision(&window),
        });
        true
    })?;
    if let Some(error) = failure.into_inner() {
        return Err(error);
    }
    Ok(WindowDiagnosticReport {
        query: query.clone(),
        windows: records.into_inner(),
    })
}

enum VirtualDesktopProbe {
    Ready(windows::Win32::UI::Shell::IVirtualDesktopManager),
    Failed(i32),
}

impl VirtualDesktopProbe {
    fn new() -> Self {
        use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance};
        use windows::Win32::UI::Shell::IVirtualDesktopManager;

        match unsafe {
            CoCreateInstance::<_, IVirtualDesktopManager>(
                &CLSID_VIRTUAL_DESKTOP_MANAGER,
                None,
                CLSCTX_INPROC_SERVER,
            )
        } {
            Ok(manager) => Self::Ready(manager),
            Err(error) => Self::Failed(error.code().0),
        }
    }

    fn read(&self, window: super::window::window_enum::WindowHandle) -> VirtualDesktopDiagnostic {
        match self {
            Self::Ready(manager) => {
                let hwnd = windows::Win32::Foundation::HWND(window);
                let is_current = unsafe { manager.IsWindowOnCurrentVirtualDesktop(hwnd) };
                let desktop_id = unsafe { manager.GetWindowDesktopId(hwnd) };
                VirtualDesktopDiagnostic {
                    is_current: is_current.as_ref().ok().map(|value| value.0 != 0),
                    is_current_hresult: is_current.err().map(|error| error.code().0),
                    desktop_id: desktop_id.as_ref().ok().map(format_guid),
                    desktop_id_hresult: desktop_id.err().map(|error| error.code().0),
                }
            }
            Self::Failed(hresult) => VirtualDesktopDiagnostic {
                is_current: None,
                is_current_hresult: Some(*hresult),
                desktop_id: None,
                desktop_id_hresult: Some(*hresult),
            },
        }
    }
}

fn format_guid(value: &windows::core::GUID) -> String {
    let [a, b, c, d, e, f, g, h] = value.data4;
    format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        value.data1, value.data2, value.data3, a, b, c, d, e, f, g, h
    )
}

#[cfg(test)]
#[path = "window_diagnostic_tests.rs"]
mod tests;
