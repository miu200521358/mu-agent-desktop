mod cloak;
mod identity;
mod inventory;
mod record;
mod report;
mod state;
mod virtual_desktop;

pub use cloak::CloakDiagnostic;
pub use identity::WindowDiagnosticIdentity;
pub use inventory::WindowInventoryDecision;
pub use record::WindowDiagnosticRecord;
pub use report::{WindowDiagnosticQuery, WindowDiagnosticReport};
pub use state::WindowDiagnosticState;
pub use virtual_desktop::VirtualDesktopDiagnostic;
