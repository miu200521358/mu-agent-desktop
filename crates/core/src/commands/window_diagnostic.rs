use crate::{
    AppError, PlatformAdapter,
    system::window_diagnostic::{WindowDiagnosticQuery, WindowDiagnosticReport},
};
use serde_json::Value;

pub struct WindowDiagnosticArgs {
    pub pid: u32,
    pub app: Option<String>,
    pub window_id: Option<String>,
}

pub fn execute(
    args: WindowDiagnosticArgs,
    adapter: &dyn PlatformAdapter,
) -> Result<Value, AppError> {
    if args.pid == 0 {
        return Err(AppError::invalid_input("--pid must be positive"));
    }
    let query = WindowDiagnosticQuery {
        pid: args.pid.into(),
        app: args.app,
        window_id: args.window_id,
    };
    let report = adapter.window_diagnostic(&query, crate::Deadline::standard()?)?;
    serialize_report(report)
}

fn serialize_report(report: WindowDiagnosticReport) -> Result<Value, AppError> {
    Ok(serde_json::to_value(report)?)
}

#[cfg(test)]
#[path = "window_diagnostic_tests.rs"]
mod tests;
