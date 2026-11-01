use clap::Parser;

use super::validate_cloaked_window_options;
use crate::cli::{Cli, Commands};

fn parse(args: &[&str]) -> (Cli, Commands) {
    let mut cli = Cli::try_parse_from(args).expect("arguments parse");
    let command = cli.command.take().expect("a command is present");
    (cli, command)
}

fn targeted(command: &[&str]) -> (Cli, Commands) {
    let mut args = vec!["mu-agent-desktop", "--pid", "7", "--cloaked-window", "w-42"];
    args.extend_from_slice(command);
    parse(&args)
}

#[test]
fn default_execution_has_no_cloak_exception() {
    let (cli, command) = parse(&["mu-agent-desktop", "click", "@s8f3k2p9:e1"]);
    assert_eq!(
        validate_cloaked_window_options(&cli, &command).unwrap(),
        None
    );
}

#[test]
fn allowlist_contains_only_the_requested_semantic_commands() {
    let commands: &[&[&str]] = &[
        &["snapshot"],
        &["screenshot"],
        &["list-surfaces"],
        &["click", "@s8f3k2p9:e1"],
        &["set-value", "@s8f3k2p9:e1", "value"],
        &["clear", "@s8f3k2p9:e1"],
        &["toggle", "@s8f3k2p9:e1"],
        &["select", "@s8f3k2p9:e1", "option"],
        &["expand", "@s8f3k2p9:e1"],
        &["collapse", "@s8f3k2p9:e1"],
        &["scroll-to", "@s8f3k2p9:e1"],
        &["get", "@s8f3k2p9:e1"],
        &["is", "@s8f3k2p9:e1"],
        &["wait", "--element", "@s8f3k2p9:e1"],
    ];
    for command_args in commands {
        let (cli, command) = targeted(command_args);
        assert!(
            validate_cloaked_window_options(&cli, &command)
                .unwrap()
                .is_some(),
            "{} is allowlisted",
            command.name()
        );
    }
}

#[test]
fn missing_pid_and_noncanonical_window_ids_are_invalid() {
    let (cli, command) = parse(&[
        "mu-agent-desktop",
        "--cloaked-window",
        "w-42",
        "click",
        "@s8f3k2p9:e1",
    ]);
    assert_eq!(
        validate_cloaked_window_options(&cli, &command)
            .unwrap_err()
            .code(),
        "INVALID_ARGS"
    );

    let (cli, command) = parse(&[
        "mu-agent-desktop",
        "--pid",
        "7",
        "--cloaked-window",
        "w-042",
        "click",
        "@s8f3k2p9:e1",
    ]);
    assert_eq!(
        validate_cloaked_window_options(&cli, &command)
            .unwrap_err()
            .code(),
        "INVALID_ARGS"
    );
}

#[test]
fn headed_and_physical_commands_fail_before_dispatch() {
    let (cli, command) = parse(&[
        "mu-agent-desktop",
        "--pid",
        "7",
        "--cloaked-window",
        "w-42",
        "--headed",
        "click",
        "@s8f3k2p9:e1",
    ]);
    assert_eq!(
        validate_cloaked_window_options(&cli, &command)
            .unwrap_err()
            .code(),
        "POLICY_DENIED"
    );

    let (cli, command) = targeted(&["mouse-click", "--xy", "10,20"]);
    assert_eq!(
        validate_cloaked_window_options(&cli, &command)
            .unwrap_err()
            .code(),
        "INVALID_ARGS"
    );
}

#[test]
fn batch_cannot_receive_a_cloaked_window_permission() {
    let (cli, command) = targeted(&["batch", "[]"]);
    assert_eq!(
        validate_cloaked_window_options(&cli, &command)
            .unwrap_err()
            .code(),
        "INVALID_ARGS"
    );
}

#[test]
fn window_diagnostic_requires_a_positive_pid() {
    let (cli, command) = parse(&["mu-agent-desktop", "window-diagnostic"]);
    assert_eq!(
        validate_cloaked_window_options(&cli, &command)
            .unwrap_err()
            .code(),
        "INVALID_ARGS"
    );

    let (cli, mut command) = parse(&["mu-agent-desktop", "--pid", "0", "window-diagnostic"]);
    if let Commands::WindowDiagnostic(args) = &mut command {
        args.pid = cli.pid;
    }
    assert_eq!(
        validate_cloaked_window_options(&cli, &command)
            .unwrap_err()
            .code(),
        "INVALID_ARGS"
    );

    let (cli, mut command) = parse(&["mu-agent-desktop", "--pid", "7", "window-diagnostic"]);
    if let Commands::WindowDiagnostic(args) = &mut command {
        args.pid = cli.pid;
    }
    assert!(validate_cloaked_window_options(&cli, &command).is_ok());
}

#[test]
fn snapshot_and_screenshot_keep_single_window_scope() {
    let (cli, command) = targeted(&["snapshot", "--surface", "taskbar"]);
    assert_eq!(
        validate_cloaked_window_options(&cli, &command)
            .unwrap_err()
            .code(),
        "INVALID_ARGS"
    );

    let (cli, command) = targeted(&["screenshot", "--screen", "0"]);
    assert_eq!(
        validate_cloaked_window_options(&cli, &command)
            .unwrap_err()
            .code(),
        "INVALID_ARGS"
    );
}
