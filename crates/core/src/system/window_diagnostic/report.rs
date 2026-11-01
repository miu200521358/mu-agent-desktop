use super::WindowDiagnosticRecord;
use crate::ProcessId;
use serde::{Deserialize, Serialize};

/// A process-scoped query; optional selectors narrow it with AND semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowDiagnosticQuery {
    pub pid: ProcessId,
    pub app: Option<String>,
    pub window_id: Option<String>,
}

/// The diagnostic query and all matching top-level windows, without refs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowDiagnosticReport {
    pub query: WindowDiagnosticQuery,
    pub windows: Vec<WindowDiagnosticRecord>,
}
