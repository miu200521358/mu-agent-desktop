use serde::{Deserialize, Serialize};

/// Public virtual-desktop API results; absent values preserve HRESULT failures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirtualDesktopDiagnostic {
    pub is_current: Option<bool>,
    pub is_current_hresult: Option<i32>,
    pub desktop_id: Option<String>,
    pub desktop_id_hresult: Option<i32>,
}
