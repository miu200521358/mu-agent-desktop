use serde::{Deserialize, Serialize};

/// A raw DWM cloak result; missing raw data means the native read failed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloakDiagnostic {
    pub known: bool,
    pub raw: Option<u32>,
    pub hresult: Option<i32>,
}
