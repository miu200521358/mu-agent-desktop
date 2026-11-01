use serde::{Deserialize, Serialize};

/// The regular agent inventory's decision and every filter that rejected it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowInventoryDecision {
    pub admitted: bool,
    pub reasons: Vec<String>,
}
