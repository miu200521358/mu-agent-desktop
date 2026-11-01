use agent_desktop_core::{WindowDiagnosticQuery, WindowDiagnosticRecord};

#[test]
fn missing_process_instance_serializes_as_explicit_unknown() {
    let record = WindowDiagnosticRecord {
        identity: agent_desktop_core::WindowDiagnosticIdentity {
            id: "w-42".into(),
            app: "Editor".into(),
            pid: 7.into(),
            process_instance: None,
        },
        state: agent_desktop_core::WindowDiagnosticState {
            visible: true,
            iconic: false,
            tool: false,
            bounds: None,
        },
        cloak: agent_desktop_core::CloakDiagnostic {
            known: false,
            raw: None,
            hresult: Some(-1),
        },
        virtual_desktop: agent_desktop_core::VirtualDesktopDiagnostic {
            is_current: None,
            is_current_hresult: Some(-2),
            desktop_id: None,
            desktop_id_hresult: Some(-2),
        },
        normal_inventory: agent_desktop_core::WindowInventoryDecision {
            admitted: true,
            reasons: Vec::new(),
        },
    };
    let json = serde_json::to_value(record).expect("diagnostic serializes");
    assert!(json["identity"]["process_instance"].is_null());
    assert_eq!(json["cloak"]["hresult"], -1);
    assert_eq!(json["virtual_desktop"]["is_current_hresult"], -2);
}

#[test]
fn query_wire_shape_keeps_both_optional_selectors() {
    let query = WindowDiagnosticQuery {
        pid: 7.into(),
        app: Some("Editor".into()),
        window_id: Some("w-42".into()),
    };
    let json = serde_json::to_value(query).expect("query serializes");
    assert_eq!(json["pid"], 7);
    assert_eq!(json["app"], "Editor");
    assert_eq!(json["window_id"], "w-42");
}
