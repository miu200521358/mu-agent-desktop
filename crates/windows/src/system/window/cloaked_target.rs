use agent_desktop_core::CloakedWindowTarget;

const KNOWN_CLOAK_BITS: u32 = 0x7;

pub(crate) struct TargetWindowEvidence {
    pub(crate) hwnd: u64,
    pub(crate) pid: u32,
    pub(crate) process_instance: Option<String>,
    pub(crate) visible: bool,
    pub(crate) nonzero: bool,
    pub(crate) tool: bool,
    pub(crate) cloak_raw: Result<u32, i32>,
}

/// Binds a one-command permission to its stored HWND, PID, and generation.
/// PID reuse alone cannot authorize a replacement process for an old ref.
pub(crate) fn matches_identity(
    target: CloakedWindowTarget,
    evidence: &TargetWindowEvidence,
    expected_instance: Option<&str>,
) -> bool {
    evidence.hwnd == target.hwnd
        && evidence.pid == target.pid
        && evidence
            .process_instance
            .as_deref()
            .is_some_and(|instance| {
                !instance.is_empty()
                    && expected_instance.is_none_or(|expected| expected == instance)
            })
}

/// The cloak exception keeps visibility, geometry, tool, and generation gates intact.
/// Unknown cloak data or bits fail closed so a partial native read cannot grant access.
pub(crate) fn eligible(
    target: CloakedWindowTarget,
    evidence: &TargetWindowEvidence,
    expected_instance: Option<&str>,
) -> bool {
    if !matches_identity(target, evidence, expected_instance) {
        return false;
    }
    if !evidence.visible || !evidence.nonzero || evidence.tool {
        return false;
    }
    evidence
        .cloak_raw
        .is_ok_and(|raw| raw & !KNOWN_CLOAK_BITS == 0)
}

#[cfg(test)]
#[path = "cloaked_target_tests.rs"]
mod tests;
