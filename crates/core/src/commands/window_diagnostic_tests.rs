use super::*;
use crate::adapter::NoopAdapter;

#[test]
fn zero_pid_is_invalid_before_the_adapter_is_called() {
    let error = execute(
        WindowDiagnosticArgs {
            pid: 0,
            app: None,
            window_id: None,
        },
        &NoopAdapter,
    )
    .unwrap_err();
    assert_eq!(error.code(), "INVALID_ARGS");
}

#[test]
fn unsupported_adapter_fails_closed() {
    let error = execute(
        WindowDiagnosticArgs {
            pid: 1,
            app: None,
            window_id: None,
        },
        &NoopAdapter,
    )
    .unwrap_err();
    assert_eq!(error.code(), "PLATFORM_NOT_SUPPORTED");
}
