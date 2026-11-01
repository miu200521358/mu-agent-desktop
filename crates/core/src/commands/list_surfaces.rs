use crate::{
    AppError,
    adapter::{PlatformAdapter, WindowFilter},
    commands::helpers::{process_identity, resolve_app},
    context::CommandContext,
};
use serde_json::{Value, json};

pub struct ListSurfacesArgs {
    pub app: Option<String>,
}

pub fn execute(args: ListSurfacesArgs, adapter: &dyn PlatformAdapter) -> Result<Value, AppError> {
    execute_with_context(args, adapter, &CommandContext::default())
}

pub fn execute_with_context(
    args: ListSurfacesArgs,
    adapter: &dyn PlatformAdapter,
    context: &CommandContext,
) -> Result<Value, AppError> {
    let deadline = crate::Deadline::standard()?;
    if let Some(target) = context.cloaked_window_target() {
        return list_permitted_surfaces(args, adapter, target, deadline);
    }
    let app = resolve_app(args.app.as_deref(), adapter, deadline)?;
    let surfaces = adapter
        .list_surfaces(process_identity(&app)?, deadline)
        .map_err(AppError::Adapter)?;
    Ok(json!({ "pid": app.pid, "surfaces": surfaces }))
}

fn list_permitted_surfaces(
    args: ListSurfacesArgs,
    adapter: &dyn PlatformAdapter,
    target: crate::CloakedWindowTarget,
    deadline: crate::Deadline,
) -> Result<Value, AppError> {
    let filter = WindowFilter {
        focused_only: false,
        app: args.app,
        cloaked_window: Some(target),
    };
    let target_id = target.window_id();
    let candidates = adapter
        .list_windows_with_filter(&filter, deadline)?
        .into_iter()
        .filter(|window| window.id == target_id && window.pid == target.pid)
        .collect();
    let window = crate::system::window_lookup::select_window(
        candidates,
        crate::AdapterError::new(
            crate::ErrorCode::WindowNotFound,
            "The permitted window is no longer an eligible top-level window",
        ),
        "Multiple windows matched the exact permitted window identity",
    )?;
    let instance = window
        .process_instance
        .as_deref()
        .filter(|instance| !instance.is_empty())
        .ok_or_else(|| {
            crate::AdapterError::new(
                crate::ErrorCode::StaleRef,
                "The permitted window has no process-instance identity",
            )
            .unsent()
        })?;
    let process = crate::ProcessIdentity::new(window.pid, instance);
    let surfaces = adapter.list_surfaces_with_filter(process, &filter, deadline)?;
    Ok(json!({ "pid": window.pid, "surfaces": surfaces }))
}
