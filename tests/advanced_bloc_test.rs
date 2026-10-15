use bloc_signals::{
    ComputedSignal, HydratedCubit, ReplayCubit, Signal, Storage,
};

#[test]
fn test_replay_cubit_undo_redo() {
    let replay = ReplayCubit::<i32, 8>::new("ReplayCounter", 0);
    assert_eq!(replay.state_value(), 0);
    assert!(!replay.can_undo());
    assert!(!replay.can_redo());

    replay.emit(10);
    replay.emit(20);
    assert_eq!(replay.state_value(), 20);
    assert!(replay.can_undo());

    assert!(replay.undo());
    assert_eq!(replay.state_value(), 10);

    assert!(replay.undo());
    assert_eq!(replay.state_value(), 0);

    assert!(replay.can_redo());
    assert!(replay.redo());
    assert_eq!(replay.state_value(), 10);
}

struct InMemoryStorage {
    stored: std::sync::Mutex<Option<i32>>,
}

impl Storage<i32> for InMemoryStorage {
    fn read(&self, _key: &str) -> Option<i32> {
        *self.stored.lock().unwrap()
    }

    fn write(&self, _key: &str, state: &i32) -> bool {
        *self.stored.lock().unwrap() = Some(*state);
        true
    }

    fn delete(&self, _key: &str) -> bool {
        *self.stored.lock().unwrap() = None;
        true
    }
}

#[test]
fn test_hydrated_cubit_persistence() {
    let storage = InMemoryStorage {
        stored: std::sync::Mutex::new(Some(42)),
    };

    let hydrated = HydratedCubit::new("HydratedCounter", 0, &storage, "counter_key");
    assert_eq!(hydrated.state_value(), 42);

    hydrated.emit(100);
    assert_eq!(hydrated.state_value(), 100);
    assert_eq!(*storage.stored.lock().unwrap(), Some(100));
}

#[test]
fn test_computed_signal() {
    let count_signal = Signal::new(5);
    let double_signal = ComputedSignal::compute(&count_signal, |x| x * 2);

    assert_eq!(double_signal.get_value(), 10);

    count_signal.set_value(10);
    double_signal.update(&count_signal, |x| x * 2);
    assert_eq!(double_signal.get_value(), 20);
}
