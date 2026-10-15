//! Global observer and telemetry for state transitions and events.

#[derive(Debug, Clone)]
pub struct Change<S> {
    pub current_state: S,
    pub next_state: S,
}

#[derive(Debug, Clone)]
pub struct Transition<E, S> {
    pub event: E,
    pub current_state: S,
    pub next_state: S,
}

pub trait BlocObserver {
    fn on_event(&self, bloc_name: &'static str, event: &dyn core::fmt::Debug) {
        let _ = (bloc_name, event);
    }

    fn on_change(&self, bloc_name: &'static str, change: &dyn core::fmt::Debug) {
        let _ = (bloc_name, change);
    }

    fn on_transition(&self, bloc_name: &'static str, transition: &dyn core::fmt::Debug) {
        let _ = (bloc_name, transition);
    }

    fn on_error(&self, bloc_name: &'static str, error: &dyn core::fmt::Debug) {
        let _ = (bloc_name, error);
    }
}

static mut OBSERVER: Option<&'static (dyn BlocObserver + Sync)> = None;

pub fn set_observer(observer: &'static (dyn BlocObserver + Sync)) {
    critical_section::with(|_| {
        unsafe {
            OBSERVER = Some(observer);
        }
    });
}

pub fn get_observer() -> Option<&'static (dyn BlocObserver + Sync)> {
    critical_section::with(|_| {
        unsafe { OBSERVER }
    })
}
