use crate::ProcessId;
use serde::{Deserialize, Serialize};

/// Process and window identity observed during read-only diagnosis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowDiagnosticIdentity {
    pub id: String,
    pub app: String,
    pub pid: ProcessId,
    pub process_instance: Option<String>,
}
