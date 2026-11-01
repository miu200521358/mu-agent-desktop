use super::{TargetWindowEvidence, eligible, matches_identity};
use agent_desktop_core::CloakedWindowTarget;

fn evidence(raw: Result<u32, i32>) -> TargetWindowEvidence {
    TargetWindowEvidence {
        hwnd: 42,
        pid: 7,
        process_instance: Some("generation-a".into()),
        visible: true,
        nonzero: true,
        tool: false,
        cloak_raw: raw,
    }
}

fn target() -> CloakedWindowTarget {
    CloakedWindowTarget { hwnd: 42, pid: 7 }
}

#[test]
fn accepts_known_cloak_flags_without_collapsing_combinations() {
    for raw in [0, 1, 2, 4, 1 | 2, 2 | 4, 1 | 2 | 4] {
        assert!(eligible(target(), &evidence(Ok(raw)), Some("generation-a")));
    }
}

#[test]
fn rejects_unknown_flag_bits_and_failed_reads() {
    assert!(!eligible(target(), &evidence(Ok(8)), Some("generation-a")));
    assert!(!eligible(
        target(),
        &evidence(Err(-1)),
        Some("generation-a")
    ));
}

#[test]
fn preserves_visibility_geometry_and_tool_exclusions() {
    let mut state = evidence(Ok(2));
    state.visible = false;
    assert!(!eligible(target(), &state, Some("generation-a")));
    let mut state = evidence(Ok(2));
    state.nonzero = false;
    assert!(!eligible(target(), &state, Some("generation-a")));
    let mut state = evidence(Ok(2));
    state.tool = true;
    assert!(!eligible(target(), &state, Some("generation-a")));
}

#[test]
fn permits_only_the_exact_handle_pid_and_process_generation() {
    assert!(eligible(target(), &evidence(Ok(2)), Some("generation-a")));
    assert!(!eligible(
        CloakedWindowTarget { hwnd: 43, pid: 7 },
        &evidence(Ok(2)),
        Some("generation-a")
    ));
    assert!(!eligible(
        CloakedWindowTarget { hwnd: 42, pid: 8 },
        &evidence(Ok(2)),
        Some("generation-a")
    ));
    assert!(!eligible(target(), &evidence(Ok(2)), Some("generation-b")));
}

#[test]
fn recycled_pid_with_a_new_process_generation_does_not_match_stored_identity() {
    let replacement = evidence(Ok(2));
    assert!(!matches_identity(
        target(),
        &replacement,
        Some("generation-b")
    ));
    assert!(!eligible(target(), &replacement, Some("generation-b")));
}

#[test]
fn missing_process_instance_fails_closed() {
    let mut state = evidence(Ok(2));
    state.process_instance = None;
    assert!(!eligible(target(), &state, None));
}
