use super::*;
use crate::{CloakedWindowTarget, WindowInfo, adapter::SnapshotSurface, refs::RefEntry};
use std::{
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

struct InventoryOnly {
    called: AtomicBool,
}

impl ObservationOps for InventoryOnly {
    fn list_windows(
        &self,
        _filter: &WindowFilter,
        _deadline: crate::Deadline,
    ) -> Result<Vec<WindowInfo>, AdapterError> {
        self.called.store(true, Ordering::SeqCst);
        Ok(Vec::new())
    }
}

#[test]
fn unsupported_adapters_reject_cloaked_window_filters_without_listing() {
    let adapter = InventoryOnly {
        called: AtomicBool::new(false),
    };
    let filter = WindowFilter {
        focused_only: false,
        app: None,
        cloaked_window: Some(CloakedWindowTarget { hwnd: 42, pid: 7 }),
    };

    let error = adapter
        .list_windows_with_filter(&filter, crate::Deadline::standard().unwrap())
        .unwrap_err();

    assert_eq!(error.code, ErrorCode::PlatformNotSupported);
    assert!(!adapter.called.load(Ordering::SeqCst));
}

struct TimedResolver {
    observed: Mutex<Option<Duration>>,
}

impl ObservationOps for TimedResolver {
    fn resolve_element_strict(
        &self,
        _entry: &RefEntry,
        deadline: crate::Deadline,
    ) -> Result<NativeHandle, AdapterError> {
        *self.observed.lock().expect("capture lock") = Some(deadline.remaining());
        Ok(NativeHandle::null())
    }
}

#[test]
fn locator_anchor_never_falls_back_to_the_public_strict_resolver() {
    let resolver = TimedResolver {
        observed: Mutex::new(None),
    };

    let error = resolver
        .resolve_locator_anchor(&entry(), crate::Deadline::standard().unwrap())
        .err()
        .expect("unsupported exact-path resolution must fail closed");

    assert_eq!(error.code, ErrorCode::PlatformNotSupported);
    assert_eq!(*resolver.observed.lock().expect("capture lock"), None);
}

fn entry() -> RefEntry {
    RefEntry {
        process: crate::RefProcess {
            pid: crate::ProcessId::new(1),
            process_instance: Some("test-instance".into()),
        },
        identity: crate::RefEntryIdentity {
            retained_object: None,
            role: "button".into(),
            name: None,
            value: None,
            description: None,
            native_id: None,
        },
        geometry: crate::RefGeometry {
            bounds_space: None,
            bounds: None,
            bounds_hash: None,
        },
        capabilities: crate::RefCapabilities {
            states: Vec::new(),
            available_actions: Vec::new(),
        },
        source: crate::RefSource {
            source_app: None,
            source_window_id: None,
            source_window_title: None,
            source_window_bounds_hash: None,
            source_surface: SnapshotSurface::Window,
        },
        scope: crate::RefScope {
            root_ref: None,
            path_is_absolute: false,
            path: Default::default(),
        },
    }
}
