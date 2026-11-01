macro_rules! adapter_cloaked_window_ops {
    () => {
        #[cfg(target_os = "windows")]
        fn window_diagnostic(
            &self,
            query: &agent_desktop_core::WindowDiagnosticQuery,
            deadline: Deadline,
        ) -> Result<agent_desktop_core::WindowDiagnosticReport, AdapterError> {
            crate::system::window_diagnostic::diagnose(query, deadline)
        }

        fn resolve_window_strict_with_filter(
            &self,
            win: &WindowInfo,
            filter: &WindowFilter,
            deadline: Deadline,
        ) -> Result<WindowInfo, AdapterError> {
            crate::system::window::window_resolve::resolve_window_strict_with_filter(
                win, filter, deadline,
            )
        }

        #[cfg(target_os = "windows")]
        fn screenshot_with_filter(
            &self,
            target: ScreenshotTarget,
            filter: &WindowFilter,
            deadline: Deadline,
        ) -> Result<ImageBuffer, AdapterError> {
            crate::system::capture::screenshot::screenshot_with_filter(target, filter, deadline)
        }
    };
}
