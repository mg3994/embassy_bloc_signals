use bloc_signals::{BlocSignal, EventTransformer, SignalOptions};

#[derive(Debug, Clone, PartialEq)]
enum TestEvent {
    Action(i32),
}

#[test]
fn test_droppable_transformer_filtering() {
    let bloc: BlocSignal<TestEvent, i32, 4> = BlocSignal::new_with_options(
        "DroppableBloc",
        0,
        SignalOptions::default(),
        EventTransformer::Droppable,
    );

    assert_eq!(bloc.transformer(), EventTransformer::Droppable);

    // Initial event added when not busy
    assert!(bloc.add(TestEvent::Action(1)));

    // Try processing event manually
    let next = bloc.try_next_event();
    assert_eq!(next, Some(TestEvent::Action(1)));
}

#[test]
fn test_sequential_transformer_queue() {
    let bloc: BlocSignal<TestEvent, i32, 4> = BlocSignal::new_with_options(
        "SequentialBloc",
        0,
        SignalOptions::default(),
        EventTransformer::Sequential,
    );

    assert_eq!(bloc.transformer(), EventTransformer::Sequential);

    assert!(bloc.add(TestEvent::Action(1)));
    assert!(bloc.add(TestEvent::Action(2)));

    assert_eq!(bloc.try_next_event(), Some(TestEvent::Action(1)));
    assert_eq!(bloc.try_next_event(), Some(TestEvent::Action(2)));
}
