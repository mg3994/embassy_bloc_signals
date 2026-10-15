//! Computed Signals for deriving reactive state from source values.

use crate::signal::Signal;

pub struct ComputedSignal<T: Clone> {
    computed_signal: Signal<T>,
}

impl<T: Clone> ComputedSignal<T> {
    pub fn compute<S1: Clone, F>(source: &Signal<S1>, compute_fn: F) -> Self
    where
        F: Fn(S1) -> T,
    {
        let initial_val = compute_fn(source.get_value());
        let computed_signal = Signal::new(initial_val);
        Self { computed_signal }
    }

    pub fn get_value(&self) -> T {
        self.computed_signal.get_value()
    }

    pub fn update<S1: Clone, F>(&self, source: &Signal<S1>, compute_fn: F) -> bool
    where
        F: Fn(S1) -> T,
    {
        let new_val = compute_fn(source.get_value());
        self.computed_signal.set_value(new_val)
    }

    #[cfg(feature = "embassy")]
    pub async fn listen_and_update<S1: Clone, F>(&self, source: &Signal<S1>, compute_fn: F)
    where
        F: Fn(S1) -> T,
    {
        if let Some(mut receiver) = source.receiver() {
            loop {
                let val = receiver.changed().await;
                let new_computed = compute_fn(val);
                self.computed_signal.set_value(new_computed);
            }
        }
    }
}
