use clap::Args;
use serde::Deserialize;

#[derive(Args, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WindowDiagnosticArgs {
    #[arg(skip)]
    #[serde(default)]
    pub pid: Option<u32>,
    #[arg(long, help = "Narrow to windows whose application name matches")]
    pub app: Option<String>,
    #[arg(long, name = "window-id", help = "Narrow to one w-<HWND> window")]
    pub window_id: Option<String>,
}
