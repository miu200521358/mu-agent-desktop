use crate::{
    AdapterError, AppError, Deadline, ErrorCode,
    adapter::{PlatformAdapter, ScreenshotTarget, WindowFilter},
    context::CommandContext,
    system::process_state::ProcessState,
};
use base64::Engine;
use serde_json::{Value, json};
use std::path::PathBuf;

const PROCESS_STATE_PROBE_BUDGET: std::time::Duration = std::time::Duration::from_secs(1);

pub struct ScreenshotArgs {
    pub app: Option<String>,
    pub window_id: Option<String>,
    pub screen: Option<usize>,
    pub output_path: Option<PathBuf>,
}

pub fn execute(args: ScreenshotArgs, adapter: &dyn PlatformAdapter) -> Result<Value, AppError> {
    execute_with_context(args, adapter, &CommandContext::default())
}

pub fn execute_with_context(
    args: ScreenshotArgs,
    adapter: &dyn PlatformAdapter,
    context: &CommandContext,
) -> Result<Value, AppError> {
    let deadline = crate::Deadline::standard()?;
    let (target, filter) = match context.cloaked_window_target() {
        Some(target) => (
            resolve_cloaked_target(&args, adapter, target, deadline)?,
            WindowFilter {
                focused_only: false,
                app: args.app.clone(),
                cloaked_window: Some(target),
            },
        ),
        None => (
            resolve_target(&args, adapter, deadline)?,
            WindowFilter::default(),
        ),
    };
    let buf = if filter.cloaked_window.is_some() {
        adapter.screenshot_with_filter(target, &filter, deadline)?
    } else {
        adapter.screenshot(target, deadline)?
    };

    if let Some(path) = args.output_path {
        crate::refs::write_user_file(&path, &buf.data)
            .map_err(|error| crate::refs::unwritable_output("the screenshot", &path, error))?;
        Ok(json!({
            "path": path.to_string_lossy(),
            "format": buf.format.as_str(),
            "width": buf.width,
            "height": buf.height,
            "scale_factor": buf.scale_factor
        }))
    } else {
        let encoded = base64::engine::general_purpose::STANDARD.encode(&buf.data);
        Ok(json!({
            "data": encoded,
            "format": buf.format.as_str(),
            "width": buf.width,
            "height": buf.height,
            "scale_factor": buf.scale_factor
        }))
    }
}

fn resolve_cloaked_target(
    args: &ScreenshotArgs,
    adapter: &dyn PlatformAdapter,
    target: crate::CloakedWindowTarget,
    deadline: crate::Deadline,
) -> Result<ScreenshotTarget, AppError> {
    if args.screen.is_some() {
        return Err(AppError::invalid_input(
            "--cloaked-window cannot target a display screenshot",
        ));
    }
    let target_id = target.window_id();
    if args
        .window_id
        .as_deref()
        .is_some_and(|window_id| window_id != target_id)
    {
        return Err(AdapterError::new(
            ErrorCode::WindowNotFound,
            "The requested window selector does not match --cloaked-window",
        )
        .unsent()
        .into());
    }
    let filter = WindowFilter {
        focused_only: false,
        app: args.app.clone(),
        cloaked_window: Some(target),
    };
    let candidates = adapter
        .list_windows_with_filter(&filter, deadline)?
        .into_iter()
        .filter(|window| {
            window.id == target_id
                && window.pid == target.pid
                && args
                    .app
                    .as_deref()
                    .is_none_or(|app| crate::app_name_matches(&window.app, app))
        })
        .collect();
    let window = crate::system::window_lookup::select_window(
        candidates,
        AdapterError::new(
            ErrorCode::WindowNotFound,
            "The permitted window is no longer an eligible top-level window",
        ),
        "Multiple windows matched the exact permitted window identity",
    )?;
    if window.process_instance.as_deref().is_none_or(str::is_empty) {
        return Err(AdapterError::new(
            ErrorCode::StaleRef,
            "The permitted window has no process-instance identity",
        )
        .unsent()
        .into());
    }
    Ok(ScreenshotTarget::ExactWindow(window))
}

fn resolve_target(
    args: &ScreenshotArgs,
    adapter: &dyn PlatformAdapter,
    deadline: crate::Deadline,
) -> Result<ScreenshotTarget, AppError> {
    if args.screen.is_some() && (args.app.is_some() || args.window_id.is_some()) {
        return Err(AppError::invalid_input_with_suggestion(
            "--screen cannot be combined with --app or --window-id",
            "Choose exactly one display target or one app/window target.",
        ));
    }
    if let Some(screen) = args.screen {
        let displays = adapter.list_displays(deadline).map_err(AppError::from)?;
        if screen >= displays.len() {
            return Err(AppError::Adapter(
                AdapterError::new(
                    ErrorCode::InvalidArgs,
                    format!(
                        "Display index {screen} out of range; {} display(s) available",
                        displays.len()
                    ),
                )
                .with_suggestion(
                    "Run 'list-displays' to see the display indexes this session has, then use one of them",
                )
                .with_details(json!({
                    "display_count": displays.len(),
                    "display_ids": displays.iter().map(|display| display.id.clone()).collect::<Vec<_>>()
                }))
                .with_disposition(crate::DeliverySemantics::not_delivered()),
            ));
        }
        return Ok(ScreenshotTarget::Display {
            index: screen,
            expected: displays[screen].clone(),
        });
    }

    if let Some(window_id) = &args.window_id {
        let expected_app = args
            .app
            .as_deref()
            .map(|name| crate::commands::helpers::resolve_app(Some(name), adapter, deadline))
            .transpose()?;
        let filter = WindowFilter {
            focused_only: false,
            app: args.app.clone(),
            cloaked_window: None,
        };
        let mut candidates = adapter
            .list_windows_with_filter(&filter, deadline)?
            .into_iter()
            .filter(|window| &window.id == window_id)
            .collect::<Vec<_>>();
        if candidates
            .iter()
            .any(|window| window.process_instance.as_deref().is_none_or(str::is_empty))
        {
            return Err(AppError::Adapter(
                AdapterError::new(
                    ErrorCode::ActionNotSupported,
                    "Matching window has incomplete process identity",
                )
                .with_suggestion("Refresh the target with 'list-windows', then retry")
                .with_disposition(crate::DeliverySemantics::not_delivered()),
            ));
        }
        if let Some(app) = expected_app.as_ref() {
            let instance = crate::commands::helpers::process_identity(app)?.instance;
            candidates.retain(|window| {
                window.pid == app.pid
                    && window.process_instance.as_deref() == Some(instance.as_str())
            });
        }
        let win = select_unique_window(
            candidates,
            window_id,
            expected_app.as_ref(),
            adapter,
            deadline,
        )?;
        return Ok(ScreenshotTarget::ExactWindow(win));
    }

    if let Some(app_name) = &args.app {
        let app = crate::commands::helpers::resolve_app(Some(app_name), adapter, deadline)?;
        let win = crate::system::window_lookup::find_window_for_process(
            crate::commands::helpers::process_identity(&app)?,
            adapter,
            deadline,
        )?;
        return Ok(ScreenshotTarget::ExactWindow(win));
    }

    Ok(ScreenshotTarget::FullScreen)
}

fn missing_window_error(
    window_id: &str,
    expected_app: Option<&crate::AppInfo>,
    adapter: &dyn PlatformAdapter,
    deadline: Deadline,
) -> AppError {
    let stopped = expected_app.filter(|app| {
        let Ok(process) = crate::commands::helpers::process_identity(app) else {
            return false;
        };
        let probe = deadline.capped(PROCESS_STATE_PROBE_BUDGET);
        adapter.process_state(process, probe).ok() == Some(ProcessState::Unresponsive)
    });
    if let Some(app) = stopped {
        return AdapterError::app_unresponsive(&app.name).into();
    }
    AdapterError::new(
        ErrorCode::WindowNotFound,
        format!("Window '{window_id}' was not found"),
    )
    .with_suggestion("Run 'list-windows' to refresh window IDs, then retry.")
    .unsent()
    .into()
}

fn select_unique_window(
    mut candidates: Vec<crate::WindowInfo>,
    window_id: &str,
    expected_app: Option<&crate::AppInfo>,
    adapter: &dyn PlatformAdapter,
    deadline: Deadline,
) -> Result<crate::WindowInfo, AppError> {
    match candidates.len() {
        0 => Err(missing_window_error(
            window_id,
            expected_app,
            adapter,
            deadline,
        )),
        1 => Ok(candidates.swap_remove(0)),
        _ => Err(AdapterError::ambiguous_target(format!(
            "Multiple windows matched id '{window_id}'"
        ))
        .with_details(json!({ "candidate_count": candidates.len() }))
        .into()),
    }
}

#[cfg(test)]
#[path = "screenshot_deadline_tests.rs"]
mod deadline_tests;
