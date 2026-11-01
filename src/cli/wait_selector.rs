use agent_desktop_core::{AppError, context::WaitSelector};

use super::{Cli, Commands};

pub(crate) fn build_wait_selector(cli: &Cli) -> Result<Option<WaitSelector>, AppError> {
    let query = cli
        .post_action_wait
        .wait_for
        .as_ref()
        .map(|raw| (raw, false))
        .or_else(|| {
            cli.post_action_wait
                .wait_for_gone
                .as_ref()
                .map(|raw| (raw, true))
        });
    let Some((query_raw, gone)) = query else {
        if cli.post_action_wait.wait_timeout.is_some() {
            return Err(AppError::invalid_input_with_suggestion(
                "--wait-timeout requires --wait-for or --wait-for-gone",
                "Add a selector wait flag or remove --wait-timeout",
            ));
        }
        return Ok(None);
    };
    Ok(Some(WaitSelector {
        query_raw: query_raw.clone(),
        gone,
        timeout_ms: cli.post_action_wait.wait_timeout.unwrap_or(30_000),
    }))
}

pub(crate) fn validate_wait_for_command(
    cmd: &Commands,
    wait: &WaitSelector,
) -> Result<(), AppError> {
    if !cmd.supports_post_action_wait() {
        return Err(AppError::invalid_input_with_suggestion(
            format!(
                "Command '{}' does not support --wait-for or --wait-for-gone",
                cmd.name()
            ),
            "Use snapshot --wait-for \"<selector>\" or a supported ref action (click, type, …).",
        ));
    }
    agent_desktop_core::commands::query::validate_selector(&wait.query_raw)?;
    Ok(())
}
