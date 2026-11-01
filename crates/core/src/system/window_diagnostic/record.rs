use super::{
    CloakDiagnostic, VirtualDesktopDiagnostic, WindowDiagnosticIdentity, WindowDiagnosticState,
    WindowInventoryDecision,
};
use serde::{Deserialize, Serialize};

/// Independent native and regular-inventory observations for one top-level window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowDiagnosticRecord {
    pub identity: WindowDiagnosticIdentity,
    pub state: WindowDiagnosticState,
    pub cloak: CloakDiagnostic,
    pub virtual_desktop: VirtualDesktopDiagnostic,
    pub normal_inventory: WindowInventoryDecision,
}
