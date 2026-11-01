use super::surface_windows;
use crate::adapter::{ActionOps, InputOps, ObservationOps, SystemOps};
use crate::{CloakedWindowTarget, Deadline, SnapshotSurface, WindowInfo, WindowState};

struct TargetAdapter {
    windows: Vec<WindowInfo>,
}

impl ObservationOps for TargetAdapter {
    fn list_windows_with_filter(
        &self,
        filter: &crate::WindowFilter,
        _deadline: Deadline,
    ) -> Result<Vec<WindowInfo>, crate::AdapterError> {
        assert_eq!(filter.cloaked_window, Some(target()));
        Ok(self
            .windows
            .iter()
            .filter(|window| {
                filter
                    .app
                    .as_deref()
                    .is_none_or(|app| crate::app_name_matches(&window.app, app))
            })
            .cloned()
            .collect())
    }

    fn list_windows(
        &self,
        filter: &crate::WindowFilter,
        _deadline: Deadline,
    ) -> Result<Vec<WindowInfo>, crate::AdapterError> {
        assert_eq!(filter.cloaked_window, Some(target()));
        Ok(self.windows.clone())
    }
}

impl ActionOps for TargetAdapter {}
impl InputOps for TargetAdapter {}
impl SystemOps for TargetAdapter {}

fn target() -> CloakedWindowTarget {
    CloakedWindowTarget { hwnd: 42, pid: 7 }
}

fn window(id: &str, app: &str) -> WindowInfo {
    WindowInfo {
        id: id.into(),
        title: app.into(),
        app: app.into(),
        pid: 7.into(),
        process_instance: Some("generation-a".into()),
        bounds: None,
        state: WindowState::default(),
    }
}

fn list(
    adapter: &TargetAdapter,
    app: Option<&str>,
    window_id: Option<&str>,
) -> Result<Vec<WindowInfo>, crate::AppError> {
    surface_windows(
        adapter,
        app,
        window_id,
        SnapshotSurface::Window,
        Some(target()),
        Deadline::after(1_000).expect("deadline is valid"),
    )
}

#[test]
fn cloaked_selector_mismatch_fails_before_inventory() {
    let adapter = TargetAdapter {
        windows: Vec::new(),
    };
    let error = list(&adapter, None, Some("w-43")).unwrap_err();
    assert_eq!(error.code(), "WINDOW_NOT_FOUND");
}

#[test]
fn cloaked_target_selects_one_exact_window_among_same_process_siblings() {
    let adapter = TargetAdapter {
        windows: vec![window("w-43", "Editor"), window("w-42", "Editor")],
    };
    let windows = list(&adapter, None, None).expect("the exact target resolves");
    assert_eq!(windows.len(), 1);
    assert_eq!(windows[0].id, "w-42");
}

#[test]
fn app_selector_is_anded_with_the_exact_cloaked_window() {
    let adapter = TargetAdapter {
        windows: vec![window("w-42", "Browser")],
    };
    let error = list(&adapter, Some("Editor"), None).unwrap_err();
    assert_eq!(error.code(), "WINDOW_NOT_FOUND");
}

#[test]
fn duplicate_exact_window_results_fail_as_ambiguous() {
    let adapter = TargetAdapter {
        windows: vec![window("w-42", "Editor"), window("w-42", "Editor")],
    };
    let error = list(&adapter, None, None).unwrap_err();
    assert_eq!(error.code(), "AMBIGUOUS_TARGET");
}
