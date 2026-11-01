use crate::ref_action::ref_action_context::RefActionContext;
use crate::{
    AdapterError, CommandContext, Deadline, ErrorCode, PlatformAdapter, RefEntry, WindowFilter,
};

pub(crate) fn resolve_context(
    target: &RefActionContext<'_>,
) -> Result<crate::adapter::NativeHandle, AdapterError> {
    resolve_element(
        target.adapter,
        target.entry,
        target.context,
        target.deadline,
    )
}

/// Keeps a snapshot ref bound to its HWND, PID, and stored process generation.
/// The adapter rechecks the live generation, so PID reuse cannot extend permission.
pub(crate) fn resolve_element(
    adapter: &dyn PlatformAdapter,
    entry: &RefEntry,
    context: &CommandContext,
    deadline: Deadline,
) -> Result<crate::adapter::NativeHandle, AdapterError> {
    resolve_element_for_target(adapter, entry, context.cloaked_window_target(), deadline)
}

pub(crate) fn resolve_element_for_target(
    adapter: &dyn PlatformAdapter,
    entry: &RefEntry,
    target: Option<crate::CloakedWindowTarget>,
    deadline: Deadline,
) -> Result<crate::adapter::NativeHandle, AdapterError> {
    if deadline.is_expired() {
        return Err(deadline.timeout_error());
    }
    let Some(target) = target else {
        return adapter.resolve_element_strict(entry, deadline);
    };
    if entry.process.pid != target.pid
        || entry.source.source_window_id.as_deref() != Some(target.window_id().as_str())
        || entry
            .process
            .process_instance
            .as_deref()
            .is_none_or(str::is_empty)
    {
        return Err(AdapterError::new(
            ErrorCode::StaleRef,
            "Ref identity does not match the permitted window process and generation",
        ));
    }
    adapter.resolve_element_strict_with_filter(
        entry,
        &WindowFilter {
            focused_only: false,
            app: None,
            cloaked_window: Some(target),
        },
        deadline,
    )
}
