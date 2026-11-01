use super::{cloak_diagnostic, matches_query, normal_inventory_decision};
use crate::system::window::window_enum::EnumeratedWindow;
use agent_desktop_core::{CloakDiagnostic, ProcessId, Rect, WindowDiagnosticQuery};

fn query(app: Option<&str>, window_id: Option<&str>) -> WindowDiagnosticQuery {
    WindowDiagnosticQuery {
        pid: ProcessId::new(7),
        app: app.map(str::to_string),
        window_id: window_id.map(str::to_string),
    }
}

fn window(visible: bool, rect: Rect, cloaked: bool, tool: bool) -> EnumeratedWindow {
    EnumeratedWindow {
        handle: std::ptr::null_mut(),
        visible,
        iconic: false,
        cloaked,
        tool,
        rect,
    }
}

#[test]
fn selectors_use_and_semantics_and_keep_only_exact_matches() {
    let query = query(Some("Editor"), Some("w-42"));
    let windows = [
        ("w-42", "Editor", 7),
        ("w-43", "Editor", 7),
        ("w-42", "Browser", 7),
        ("w-42", "Editor", 8),
    ];
    let matches = windows
        .into_iter()
        .filter(|(id, app, pid)| matches_query(&query, id, app, *pid))
        .collect::<Vec<_>>();
    assert_eq!(matches, [("w-42", "Editor", 7)]);
}

#[test]
fn process_only_query_keeps_multiple_windows_for_the_pid() {
    let query = query(None, None);
    let ids = ["w-42", "w-43", "w-44"]
        .into_iter()
        .filter(|id| matches_query(&query, id, "Editor", 7))
        .collect::<Vec<_>>();
    assert_eq!(ids, ["w-42", "w-43", "w-44"]);
}

#[test]
fn cloak_diagnostics_preserve_raw_flags_and_failures() {
    for raw in [0, 1, 2, 4, 3, 5, 6, 7] {
        assert_eq!(
            cloak_diagnostic(Ok(raw)),
            CloakDiagnostic {
                known: true,
                raw: Some(raw),
                hresult: None,
            }
        );
    }
    assert_eq!(
        cloak_diagnostic(Ok(8)),
        CloakDiagnostic {
            known: false,
            raw: Some(8),
            hresult: None,
        }
    );
    assert_eq!(
        cloak_diagnostic(Err(-1)),
        CloakDiagnostic {
            known: false,
            raw: None,
            hresult: Some(-1),
        }
    );
}

#[test]
fn normal_inventory_decision_reports_every_unchanged_filter() {
    let visible_window = window(
        true,
        Rect {
            x: 10.0,
            y: 20.0,
            width: 640.0,
            height: 480.0,
        },
        false,
        false,
    );
    assert_eq!(
        normal_inventory_decision(&visible_window),
        agent_desktop_core::WindowInventoryDecision {
            admitted: true,
            reasons: Vec::new(),
        }
    );
    let rejected = window(
        false,
        Rect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 1.0,
        },
        true,
        true,
    );
    let decision = normal_inventory_decision(&rejected);
    assert!(!decision.admitted);
    assert_eq!(
        decision.reasons,
        ["invisible", "zero_size", "cloaked", "tool_window"]
    );
}

#[test]
fn each_non_cloak_inventory_gate_rejects_on_its_own() {
    let bounds = Rect {
        x: 10.0,
        y: 20.0,
        width: 640.0,
        height: 480.0,
    };
    let cases = [
        (window(false, bounds, false, false), "invisible"),
        (
            window(
                true,
                Rect {
                    width: 0.0,
                    ..bounds
                },
                false,
                false,
            ),
            "zero_size",
        ),
        (window(true, bounds, false, true), "tool_window"),
    ];

    for (window, reason) in cases {
        let decision = normal_inventory_decision(&window);
        assert!(!decision.admitted);
        assert_eq!(decision.reasons, [reason]);
    }
}
