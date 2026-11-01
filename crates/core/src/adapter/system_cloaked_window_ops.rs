macro_rules! system_cloaked_window_ops {
    () => {
        /// Reads Windows window identity and cloak/desktop diagnostics without issuing refs.
        fn window_diagnostic(
            &self,
            _query: &WindowDiagnosticQuery,
            _deadline: Deadline,
        ) -> Result<WindowDiagnosticReport, AdapterError> {
            Err(AdapterError::not_supported("window_diagnostic"))
        }

        /// Captures an exact window only when the adapter can enforce the supplied filter.
        fn screenshot_with_filter(
            &self,
            target: ScreenshotTarget,
            filter: &WindowFilter,
            deadline: Deadline,
        ) -> Result<ImageBuffer, AdapterError> {
            if filter.cloaked_window.is_some() {
                return Err(AdapterError::not_supported("screenshot_with_filter"));
            }
            self.screenshot(target, deadline)
        }

        /// Revalidates one exact window under the caller's purpose-specific permission.
        fn resolve_window_strict_with_filter(
            &self,
            window: &WindowInfo,
            filter: &WindowFilter,
            deadline: Deadline,
        ) -> Result<WindowInfo, AdapterError> {
            if filter.cloaked_window.is_some() {
                return Err(AdapterError::not_supported(
                    "resolve_window_strict_with_filter",
                ));
            }
            self.resolve_window_strict(window, deadline)
        }
    };
}
