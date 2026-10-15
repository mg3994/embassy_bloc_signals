//! Global observer and telemetry for state transitions and events.

use critical_section::Mutex;
use core::cell::RefCell;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Change<S> {
    pub current_state: S,
    pub next_state: S,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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

static OBSERVER: Mutex<RefCell<Option<&'static (dyn BlocObserver + Sync)>>> = Mutex::new(RefCell::new(None));

pub fn set_observer(observer: &'static (dyn BlocObserver + Sync)) {
    critical_section::with(|cs| {
        *OBSERVER.borrow(cs).borrow_mut() = Some(observer);
    });
}

pub fn get_observer() -> Option<&'static (dyn BlocObserver + Sync)> {
    critical_section::with(|cs| {
        *OBSERVER.borrow(cs).borrow()
    })
}

#[cfg(feature = "log")]
pub struct LogBlocObserver;

#[cfg(feature = "log")]
impl BlocObserver for LogBlocObserver {
    fn on_event(&self, bloc_name: &'static str, event: &dyn core::fmt::Debug) {
        log::info!("[{}] Event: {:?}", bloc_name, event);
    }

    fn on_change(&self, bloc_name: &'static str, change: &dyn core::fmt::Debug) {
        log::info!("[{}] Change: {:?}", bloc_name, change);
    }

    fn on_transition(&self, bloc_name: &'static str, transition: &dyn core::fmt::Debug) {
        log::info!("[{}] Transition: {:?}", bloc_name, transition);
    }

    fn on_error(&self, bloc_name: &'static str, error: &dyn core::fmt::Debug) {
        log::error!("[{}] Error: {:?}", bloc_name, error);
    }
}
