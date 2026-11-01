use super::CloakedWindowTarget;

#[test]
fn parses_one_canonical_window_and_pid() {
    assert_eq!(
        CloakedWindowTarget::parse("w-42", 7).unwrap(),
        CloakedWindowTarget { hwnd: 42, pid: 7 }
    );
}

#[test]
fn rejects_malformed_or_non_positive_target_values() {
    for (window_id, pid) in [
        ("42", 7),
        ("w-0", 7),
        ("w-01", 7),
        ("w--1", 7),
        ("w-1x", 7),
        ("w-18446744073709551616", 7),
        ("w-42", 0),
    ] {
        assert!(CloakedWindowTarget::parse(window_id, pid).is_err());
    }
}
