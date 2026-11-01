use agent_desktop_core::{
    AdapterError, AppError, CloakedWindowTarget, DeliverySemantics, ErrorCode,
};

use super::{Cli, Commands};

/// Keeps a cloak exception on one semantic command and refuses headed paths before dispatch.
pub(crate) fn validate_cloaked_window_options(
    cli: &Cli,
    cmd: &Commands,
) -> Result<Option<CloakedWindowTarget>, AppError> {
    let diagnostic = matches!(cmd, Commands::WindowDiagnostic(_));
    if let Some(window_id) = cli.cloaked_window.as_deref() {
        if !supports_cloaked_target(cmd) {
            return Err(AppError::invalid_input(
                "--cloaked-window is not supported by this command",
            ));
        }
        let pid = cli
            .pid
            .ok_or_else(|| AppError::invalid_input("--cloaked-window requires --pid"))?;
        if cli.interaction.headed || cli.visual_debug.debug || cli.visual_debug.screenshot.is_some()
        {
            return Err(AdapterError::new(
                ErrorCode::PolicyDenied,
                "--cloaked-window cannot be combined with headed or visual-debug behavior",
            )
            .with_disposition(DeliverySemantics::not_delivered())
            .into());
        }
        match cmd {
            Commands::Snapshot(args) if args.surface != crate::cli_args::Surface::Window => {
                return Err(AppError::invalid_input(
                    "--cloaked-window supports the window snapshot surface only",
                ));
            }
            Commands::Screenshot(args) if args.screen.is_some() => {
                return Err(AppError::invalid_input(
                    "--cloaked-window cannot target a display screenshot",
                ));
            }
            Commands::Wait(args) if args.mode.element.is_none() => {
                return Err(AppError::invalid_input(
                    "--cloaked-window supports wait only with a ref element",
                ));
            }
            _ => {}
        }
        return CloakedWindowTarget::parse(window_id, pid)
            .map(Some)
            .map_err(AppError::Adapter);
    }
    if let Some(pid) = cli.pid {
        if !diagnostic {
            return Err(AppError::invalid_input(
                "--pid is supported only by window-diagnostic or with --cloaked-window",
            ));
        }
        if pid == 0 {
            return Err(AppError::invalid_input("--pid must be positive"));
        }
    }
    if diagnostic
        && match cmd {
            Commands::WindowDiagnostic(args) => args.pid.is_none_or(|pid| pid == 0),
            _ => true,
        }
    {
        return Err(AppError::invalid_input(
            "window-diagnostic requires a positive --pid",
        ));
    }
    Ok(None)
}

fn supports_cloaked_target(cmd: &Commands) -> bool {
    matches!(
        cmd,
        Commands::Snapshot(_)
            | Commands::Screenshot(_)
            | Commands::ListSurfaces(_)
            | Commands::Click(_)
            | Commands::SetValue(_)
            | Commands::Clear(_)
            | Commands::Toggle(_)
            | Commands::Select(_)
            | Commands::Expand(_)
            | Commands::Collapse(_)
            | Commands::ScrollTo(_)
            | Commands::Get(_)
            | Commands::Is(_)
            | Commands::Wait(_)
    )
}

#[cfg(test)]
#[path = "cloaked_window_tests.rs"]
mod tests;
