use super::super::element::UIAElement;
use super::{stale_ref_error, verify_stored_failure_error};
use agent_desktop_core::{AdapterError, Deadline, RefEntry, WindowFilter};

pub(crate) fn resolve_window_root_with_filter(
    entry: &RefEntry,
    filter: &WindowFilter,
    deadline: Deadline,
) -> Result<UIAElement, AdapterError> {
    let window_id = entry
        .source
        .source_window_id
        .as_deref()
        .ok_or_else(|| stale_ref_error(entry))?;
    let window = agent_desktop_core::WindowInfo {
        id: window_id.to_string(),
        title: entry.source.source_window_title.clone().unwrap_or_default(),
        app: entry.source.source_app.clone().unwrap_or_default(),
        pid: entry.process.pid,
        process_instance: entry.process.process_instance.clone(),
        bounds: None,
        state: Default::default(),
    };
    crate::system::window::window_identity::WindowIdentityEvidence::from_info(
        crate::system::window::window_ops::parse_handle(&window.id),
        &window,
    )
    .ok_or_else(|| stale_ref_error(entry))?
    .verify_stored()
    .map_err(|_| verify_stored_failure_error(entry, deadline))?;

    crate::tree::surfaces::surface_root_with_filter(
        agent_desktop_core::ObservationRoot::Window(&window),
        entry.source.source_surface,
        filter,
        deadline,
    )
}
