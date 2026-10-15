use bloc_signals::{
    get_observer, set_observer, BlocObserver, BlocSignal, CubitSignal, Equality, Signal,
    SignalOptions,
};

#[test]
fn test_cubit_signal_basic() {
    let cubit = CubitSignal::new("CounterCubit", 0);
    assert_eq!(cubit.state_value(), 0);

    assert!(cubit.emit(1));
    assert_eq!(cubit.state_value(), 1);

    assert!(cubit.emit(2));
    assert_eq!(cubit.state_value(), 2);
}

#[test]
fn test_cubit_emit_deduplication() {
    let cubit = CubitSignal::new("CounterCubit", 10);
    assert_eq!(cubit.state_value(), 10);

    // emit_if_not_eq with identical state should return false
    assert!(!cubit.emit_if_not_eq(10));
    assert_eq!(cubit.state_value(), 10);

    assert!(cubit.emit_if_not_eq(20));
    assert_eq!(cubit.state_value(), 20);
}

#[derive(Debug, Clone, PartialEq)]
enum CounterEvent {
    Increment,
    Decrement,
}

#[test]
fn test_bloc_signal_event_queue() {
    let bloc: BlocSignal<CounterEvent, i32, 8> = BlocSignal::new("CounterBloc", 0);
    assert_eq!(bloc.state_value(), 0);

    assert!(bloc.add(CounterEvent::Increment));
    assert!(bloc.add(CounterEvent::Decrement));

    assert!(bloc.emit(CounterEvent::Increment, 1));
    assert_eq!(bloc.state_value(), 1);
}

struct TestObserver;
impl BlocObserver for TestObserver {}

static OBSERVER_INSTANCE: TestObserver = TestObserver;

#[test]
fn test_observer_registration() {
    set_observer(&OBSERVER_INSTANCE);
    assert!(get_observer().is_some());
}
