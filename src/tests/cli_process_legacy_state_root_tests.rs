use super::binary;

#[test]
fn legacy_state_root_env_is_ignored() {
    let output = binary()
        .arg("version")
        .env_remove("MU_AGENT_DESKTOP_HOME")
        .env("AGENT_DESKTOP_HOME", "relative/legacy-root")
        .output()
        .expect("binary starts");
    let envelope: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout is one JSON envelope");

    assert!(output.status.success());
    assert_eq!(envelope["ok"], true);
}

#[test]
fn invalid_state_root_env_fails_batch_as_single_envelope() {
    let output = binary()
        .args(["batch", "[]"])
        .env("MU_AGENT_DESKTOP_HOME", "relative/not-absolute")
        .output()
        .expect("binary starts");
    let envelope: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout is one JSON envelope");

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(envelope["ok"], false);
    assert_eq!(envelope["command"], "batch");
    assert_eq!(envelope["error"]["code"], "INVALID_ARGS");
    assert!(
        envelope.get("data").is_none(),
        "batch must fail as one error envelope, no entry results"
    );
}
