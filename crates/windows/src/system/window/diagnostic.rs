use agent_desktop_core::{WindowDiagnosticQuery, WindowInventoryDecision, app_name_matches};

use super::window_enum::EnumeratedWindow;

const KNOWN_CLOAK_BITS: u32 = 0x7;

pub(crate) fn matches_query(query: &WindowDiagnosticQuery, id: &str, app: &str, pid: u32) -> bool {
    query.pid.get() == pid
        && query
            .window_id
            .as_deref()
            .is_none_or(|expected| expected == id)
        && query
            .app
            .as_deref()
            .is_none_or(|expected| app_name_matches(app, expected))
}

pub(crate) fn cloak_diagnostic(raw: Result<u32, i32>) -> agent_desktop_core::CloakDiagnostic {
    match raw {
        Ok(raw) => agent_desktop_core::CloakDiagnostic {
            known: raw & !KNOWN_CLOAK_BITS == 0,
            raw: Some(raw),
            hresult: None,
        },
        Err(hresult) => agent_desktop_core::CloakDiagnostic {
            known: false,
            raw: None,
            hresult: Some(hresult),
        },
    }
}

pub(crate) fn normal_inventory_decision(window: &EnumeratedWindow) -> WindowInventoryDecision {
    let mut reasons = Vec::new();
    if !window.visible {
        reasons.push("invisible".to_string());
    }
    if window.is_zero_sized() {
        reasons.push("zero_size".to_string());
    }
    if window.cloaked {
        reasons.push("cloaked".to_string());
    }
    if window.tool {
        reasons.push("tool_window".to_string());
    }
    WindowInventoryDecision {
        admitted: reasons.is_empty(),
        reasons,
    }
}

#[cfg(test)]
#[path = "diagnostic_tests.rs"]
mod tests;
