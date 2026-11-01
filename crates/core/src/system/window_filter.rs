#[derive(Debug, Clone, Default)]
pub struct WindowFilter {
    pub focused_only: bool,
    /// Exact application identity enforced by the platform adapter. Adapters
    /// may resolve this against display names, bundle identifiers, or PIDs.
    pub app: Option<String>,
    /// One-command exception for exactly one HWND and PID. Adapters must still
    /// require a known process generation and all non-cloak inventory gates;
    /// unsupported adapters reject this field so the permission fails closed.
    pub cloaked_window: Option<super::cloaked_window_target::CloakedWindowTarget>,
}
