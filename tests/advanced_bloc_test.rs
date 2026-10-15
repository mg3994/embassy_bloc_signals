use bloc_signals::{
    ComputedSignal, ComputedSignal2, HydratedCubit, ReplayCubit, Signal, Storage,
};
use static_cell::StaticCell;

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

#[test]
fn test_computed_signal2_multi_source() {
    let sig1 = Signal::new(10);
    let sig2 = Signal::new(20);

    let sum_sig = ComputedSignal2::compute(&sig1, &sig2, |a, b| a + b);
    assert_eq!(sum_sig.get_value(), 30);

    sig1.set_value(15);
    sum_sig.update(&sig1, &sig2, |a, b| a + b);
    assert_eq!(sum_sig.get_value(), 35);
}

static COUNT_SIGNAL: StaticCell<Signal<i32>> = StaticCell::new();
static TRIPLE_SIGNAL: StaticCell<ComputedSignal<i32>> = StaticCell::new();

#[tokio::test]
async fn test_computed_signal_auto_update() {
    let count_signal: &'static Signal<i32> = COUNT_SIGNAL.init(Signal::new(3));
    let triple_signal: &'static ComputedSignal<i32> = TRIPLE_SIGNAL.init(ComputedSignal::compute(count_signal, |x| x * 3));

    assert_eq!(triple_signal.get_value(), 9);

    let handle = tokio::spawn(async move {
        triple_signal.listen_and_update(count_signal, |x| x * 3).await;
    });

    tokio::task::yield_now().await;
    count_signal.set_value(5);
    tokio::task::yield_now().await;

    assert_eq!(triple_signal.get_value(), 15);
    handle.abort();
}
