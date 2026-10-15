//! Cubit state container bridging synchronous method-driven logic with reactive signals.

use crate::observer::{get_observer, Change};
use crate::signal::{ReadonlySignal, Signal, SignalOptions};

pub struct CubitSignal<S: Clone> {
    name: &'static str,
    signal: Signal<S>,
}

impl<S: Clone> CubitSignal<S> {
    pub fn new(name: &'static str, initial_state: S) -> Self {
        Self::new_with_options(name, initial_state, SignalOptions::default())
    }

    pub fn new_with_options(
        name: &'static str,
        initial_state: S,
        mut options: SignalOptions<S>,
    ) -> Self {
        options.name = name;
        Self {
            name,
            signal: Signal::new_with_options(initial_state, options),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn state(&self) -> ReadonlySignal<'_, S> {
        ReadonlySignal::new(&self.signal)
    }

    pub fn state_value(&self) -> S {
        self.signal.get_value()
    }

    pub fn emit(&self, new_state: S) -> bool {
        let current_state = self.state_value();

        if self.signal.set_value(new_state.clone()) {
            if let Some(observer) = get_observer() {
                let change = Change {
                    current_state: current_state.clone(),
                    next_state: new_state.clone(),
                };
                observer.on_change(self.name, &change);
            }
            true
        } else {
            false
        }
    }

    pub fn emit_if_not_eq(&self, new_state: S) -> bool
    where
        S: PartialEq,
    {
        let current_state = self.state_value();
        if current_state == new_state {
            return false;
        }

        if self.signal.set_value_if_not_eq(new_state.clone()) {
            if let Some(observer) = get_observer() {
                let change = Change {
                    current_state: current_state.clone(),
                    next_state: new_state.clone(),
                };
                observer.on_change(self.name, &change);
            }
            true
        } else {
            false
        }
    }
}
