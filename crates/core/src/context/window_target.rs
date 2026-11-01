use super::CommandContext;

impl CommandContext {
    /// Grants this command access to one exactly identified window.
    pub fn with_cloaked_window_target(mut self, target: crate::CloakedWindowTarget) -> Self {
        self.options.cloaked_window_target = Some(target);
        self
    }

    /// Returns the one-command window permission, if the CLI supplied it.
    pub fn cloaked_window_target(&self) -> Option<crate::CloakedWindowTarget> {
        self.options.cloaked_window_target
    }
}
