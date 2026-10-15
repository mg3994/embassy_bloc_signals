//! Reactive Signals primitives for `bloc_signals`.

use critical_section::Mutex;
use core::cell::RefCell;

#[cfg(feature = "embassy")]
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
#[cfg(feature = "embassy")]
use embassy_sync::watch::Watch;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Equality<T> {
    AlwaysUpdate,
    Custom(fn(&T, &T) -> bool),
}

pub struct SignalOptions<T> {
    pub name: &'static str,
    pub equality: Equality<T>,
}

impl<T> Default for SignalOptions<T> {
    fn default() -> Self {
        Self {
            name: "Signal",
            equality: Equality::AlwaysUpdate,
        }
    }
}

pub struct Signal<T: Clone> {
    options: SignalOptions<T>,
    inner: Mutex<RefCell<T>>,
    #[cfg(feature = "embassy")]
    watch: Watch<CriticalSectionRawMutex, T, 8>,
}

// CriticalSection Mutex + RefCell makes Signal Send + Sync safely in embedded environments
unsafe impl<T: Clone + Send> Send for Signal<T> {}
unsafe impl<T: Clone + Send> Sync for Signal<T> {}

impl<T: Clone> Signal<T> {
    pub fn new(initial_value: T) -> Self {
        Self::new_with_options(initial_value, SignalOptions::default())
    }

    pub fn new_with_options(initial_value: T, options: SignalOptions<T>) -> Self {
        #[cfg(feature = "embassy")]
        {
            let watch = Watch::new();
            watch.sender().send(initial_value.clone());
            Self {
                options,
                inner: Mutex::new(RefCell::new(initial_value)),
                watch,
            }
        }
        #[cfg(not(feature = "embassy"))]
        {
            Self {
                options,
                inner: Mutex::new(RefCell::new(initial_value)),
            }
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.options.name
    }

    pub fn get_value(&self) -> T {
        critical_section::with(|cs| self.inner.borrow(cs).borrow().clone())
    }

    pub fn set_value(&self, new_value: T) -> bool {
        let current_value = self.get_value();

        let is_equal = match &self.options.equality {
            Equality::AlwaysUpdate => false,
            Equality::Custom(f) => f(&current_value, &new_value),
        };

        if is_equal {
            return false;
        }

        critical_section::with(|cs| {
            *self.inner.borrow(cs).borrow_mut() = new_value.clone();
        });

        #[cfg(feature = "embassy")]
        {
            self.watch.sender().send(new_value);
        }

        true
    }

    pub fn set_value_if_not_eq(&self, new_value: T) -> bool
    where
        T: PartialEq,
    {
        let current_value = self.get_value();
        if current_value == new_value {
            return false;
        }

        critical_section::with(|cs| {
            *self.inner.borrow(cs).borrow_mut() = new_value.clone();
        });

        #[cfg(feature = "embassy")]
        {
            self.watch.sender().send(new_value);
        }

        true
    }

    #[cfg(feature = "embassy")]
    pub fn receiver(&self) -> Option<embassy_sync::watch::Receiver<'_, CriticalSectionRawMutex, T, 8>> {
        self.watch.receiver().ok()
    }
}

pub struct ReadonlySignal<'a, T: Clone> {
    signal: &'a Signal<T>,
}

impl<'a, T: Clone> ReadonlySignal<'a, T> {
    pub fn new(signal: &'a Signal<T>) -> Self {
        Self { signal }
    }

    pub fn get_value(&self) -> T {
        self.signal.get_value()
    }

    #[cfg(feature = "embassy")]
    pub fn receiver(&self) -> Option<embassy_sync::watch::Receiver<'a, CriticalSectionRawMutex, T, 8>> {
        self.signal.receiver()
    }
}
