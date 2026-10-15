//! Replay state history (undo/redo) tracking for Cubits and Blocs.

use crate::cubit::CubitSignal;
use critical_section::Mutex;
use core::cell::RefCell;
use heapless::HistoryBuffer;

pub struct ReplayCubit<S: Clone, const CAP: usize = 16> {
    cubit: CubitSignal<S>,
    past: Mutex<RefCell<HistoryBuffer<S, CAP>>>,
    future: Mutex<RefCell<heapless::Vec<S, CAP>>>,
}

impl<S: Clone + core::fmt::Debug, const CAP: usize> ReplayCubit<S, CAP> {
    pub fn new(name: &'static str, initial_state: S) -> Self {
        Self {
            cubit: CubitSignal::new(name, initial_state),
            past: Mutex::new(RefCell::new(HistoryBuffer::new())),
            future: Mutex::new(RefCell::new(heapless::Vec::new())),
        }
    }

    pub fn cubit(&self) -> &CubitSignal<S> {
        &self.cubit
    }

    pub fn state_value(&self) -> S {
        self.cubit.state_value()
    }

    pub fn emit(&self, new_state: S) -> bool {
        let current_state = self.cubit.state_value();
        if self.cubit.emit(new_state) {
            critical_section::with(|cs| {
                self.past.borrow(cs).borrow_mut().write(current_state);
                self.future.borrow(cs).borrow_mut().clear();
            });
            true
        } else {
            false
        }
    }

    pub fn can_undo(&self) -> bool {
        critical_section::with(|cs| {
            !self.past.borrow(cs).borrow().is_empty()
        })
    }

    pub fn can_redo(&self) -> bool {
        critical_section::with(|cs| {
            !self.future.borrow(cs).borrow().is_empty()
        })
    }

    pub fn undo(&self) -> bool {
        if !self.can_undo() {
            return false;
        }

        let current_state = self.cubit.state_value();
        let prev_state = critical_section::with(|cs| {
            let mut past = self.past.borrow(cs).borrow_mut();
            let mut future = self.future.borrow(cs).borrow_mut();

            let history: heapless::Vec<S, CAP> = past.oldest_ordered().cloned().collect();
            if let Some((last, remaining)) = history.split_last() {
                past.clear();
                for item in remaining {
                    past.write(item.clone());
                }
                let _ = future.push(current_state);
                Some(last.clone())
            } else {
                None
            }
        });

        if let Some(prev) = prev_state {
            self.cubit.emit(prev)
        } else {
            false
        }
    }

    pub fn redo(&self) -> bool {
        if !self.can_redo() {
            return false;
        }

        let current_state = self.cubit.state_value();
        let next_state = critical_section::with(|cs| {
            let mut past = self.past.borrow(cs).borrow_mut();
            let mut future = self.future.borrow(cs).borrow_mut();

            if let Some(next) = future.pop() {
                past.write(current_state);
                Some(next)
            } else {
                None
            }
        });

        if let Some(next) = next_state {
            self.cubit.emit(next)
        } else {
            false
        }
    }
}
