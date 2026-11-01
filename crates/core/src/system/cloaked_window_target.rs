use crate::{AdapterError, ErrorCode};

/// A one-command permission scoped to one HWND and its owning process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CloakedWindowTarget {
    pub hwnd: u64,
    pub pid: u32,
}

impl CloakedWindowTarget {
    /// Parses the exact CLI spelling and rejects aliases that could blur target identity.
    pub fn parse(window_id: &str, pid: u32) -> Result<Self, AdapterError> {
        let digits = window_id.strip_prefix("w-").ok_or_else(invalid_target)?;
        if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) || pid == 0 {
            return Err(invalid_target());
        }
        let hwnd = digits.parse::<u64>().map_err(|_| invalid_target())?;
        if hwnd == 0 || window_id != format!("w-{hwnd}") {
            return Err(invalid_target());
        }
        Ok(Self { hwnd, pid })
    }

    /// Returns the canonical public window id used by window inventories.
    pub fn window_id(self) -> String {
        format!("w-{}", self.hwnd)
    }
}

fn invalid_target() -> AdapterError {
    AdapterError::new(
        ErrorCode::InvalidArgs,
        "--cloaked-window must be a canonical w-<positive decimal HWND> and --pid must be positive",
    )
}

#[cfg(test)]
#[path = "cloaked_window_target_tests.rs"]
mod tests;
