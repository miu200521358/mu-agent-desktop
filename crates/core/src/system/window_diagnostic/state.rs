use crate::Rect;
use serde::{Deserialize, Serialize};

/// Native visibility, iconic state, tool status, and bounds from one HWND.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowDiagnosticState {
    pub visible: bool,
    pub iconic: bool,
    pub tool: bool,
    pub bounds: Option<Rect>,
}
