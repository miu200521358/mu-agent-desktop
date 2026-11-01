use std::path::PathBuf;

use clap::Parser;

use super::{Commands, post_action_wait::PostActionWaitArgs};
use crate::cli_args::interaction::InteractionArgs;

const BEFORE_HELP: &str = include_str!("help_before.txt");
const AFTER_HELP: &str = include_str!("help_after.txt");

#[derive(Parser, Debug)]
#[command(
    name = "mu-agent-desktop",
    version,
    about = "Reliable computer use for AI agents — see and operate desktop apps",
    long_about = None,
    before_help = BEFORE_HELP,
    after_help = AFTER_HELP,
)]
pub(crate) struct Cli {
    #[command(flatten)]
    pub visual_debug: crate::visual_debug::options::DebugOptions,
    #[command(flatten)]
    pub identity: super::identity::IdentityArgs,
    #[arg(
        long,
        global = true,
        help = "Append reliability trace JSONL to this path"
    )]
    pub trace: Option<PathBuf>,
    #[arg(
        long,
        global = true,
        help = "Fail on trace setup/pre-action write errors"
    )]
    pub trace_strict: bool,
    #[arg(
        long,
        global = true,
        value_name = "PID",
        help = "Process ID for Windows window-diagnostic or --cloaked-window"
    )]
    pub pid: Option<u32>,
    #[arg(
        long,
        global = true,
        value_name = "WINDOW_ID",
        help = "Permit one exact cloaked Windows window; requires --pid"
    )]
    pub cloaked_window: Option<String>,
    #[command(flatten)]
    pub interaction: InteractionArgs,
    #[command(flatten)]
    pub post_action_wait: PostActionWaitArgs,
    #[command(subcommand)]
    pub command: Option<Commands>,
}
