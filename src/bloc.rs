//! Bloc event-driven state container and async event processing.

#[cfg(feature = "embassy")]
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
#[cfg(feature = "embassy")]
use embassy_sync::channel::Channel;

use crate::observer::{get_observer, Change, Transition};
use crate::signal::{ReadonlySignal, Signal, SignalOptions};
use crate::transformer::EventTransformer;

pub struct BlocSignal<E: Clone, S: Clone, const N: usize = 16> {
    name: &'static str,
    signal: Signal<S>,
    transformer: EventTransformer,
    #[cfg(feature = "embassy")]
    event_channel: Channel<CriticalSectionRawMutex, E, N>,
    #[cfg(not(feature = "embassy"))]
    event_queue: core::cell::RefCell<heapless::Vec<E, N>>,
}

impl<E: Clone, S: Clone, const N: usize> BlocSignal<E, S, N> {
    pub fn new(name: &'static str, initial_state: S) -> Self {
        Self::new_with_options(name, initial_state, SignalOptions::default(), EventTransformer::Sequential)
    }

    pub fn new_with_options(
        name: &'static str,
        initial_state: S,
        mut options: SignalOptions<S>,
        transformer: EventTransformer,
    ) -> Self {
        options.name = name;
        Self {
            name,
            signal: Signal::new_with_options(initial_state, options),
            transformer,
            #[cfg(feature = "embassy")]
            event_channel: Channel::new(),
            #[cfg(not(feature = "embassy"))]
            event_queue: core::cell::RefCell::new(heapless::Vec::new()),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn transformer(&self) -> EventTransformer {
        self.transformer
    }

    pub fn state(&self) -> ReadonlySignal<'_, S> {
        ReadonlySignal::new(&self.signal)
    }

    pub fn state_value(&self) -> S {
        self.signal.get_value()
    }

    pub fn add(&self, event: E) -> bool
    where
        E: core::fmt::Debug,
    {
        if let Some(observer) = get_observer() {
            observer.on_event(self.name, &event);
        }

        #[cfg(feature = "embassy")]
        {
            match self.transformer {
                EventTransformer::Droppable => {
                    self.event_channel.try_send(event).is_ok()
                }
                _ => {
                    // For Sequential, Restartable, Concurrent, enqueue event
                    self.event_channel.try_send(event).is_ok()
                }
            }
        }
        #[cfg(not(feature = "embassy"))]
        {
            let mut q = self.event_queue.borrow_mut();
            if q.len() < N {
                q.push(event).is_ok()
            } else {
                false
            }
        }
    }

    pub fn emit(&self, event: E, new_state: S) -> bool
    where
        E: core::fmt::Debug,
        S: core::fmt::Debug,
    {
        let current_state = self.state_value();

        if self.signal.set_value(new_state.clone()) {
            if let Some(observer) = get_observer() {
                let change = Change {
                    current_state: current_state.clone(),
                    next_state: new_state.clone(),
                };
                observer.on_change(self.name, &change);

                let transition = Transition {
                    event,
                    current_state,
                    next_state: new_state,
                };
                observer.on_transition(self.name, &transition);
            }
            true
        } else {
            false
        }
    }

    #[cfg(feature = "embassy")]
    pub async fn next_event(&self) -> E {
        self.event_channel.receive().await
    }

    #[cfg(feature = "embassy")]
    pub fn try_next_event(&self) -> Option<E> {
        self.event_channel.try_receive().ok()
    }

    #[cfg(feature = "embassy")]
    pub async fn handle_events<F, Fut>(&self, mut handler: F)
    where
        F: FnMut(E, S) -> Fut,
        Fut: core::future::Future<Output = Option<S>>,
        E: core::fmt::Debug,
        S: core::fmt::Debug,
    {
        loop {
            let event = self.next_event().await;
            let current_state = self.state_value();

            if let Some(next_state) = handler(event.clone(), current_state).await {
                self.emit(event, next_state);
            }
        }
    }
}
