//! Hydration and persistent state storage interface for Cubits and Blocs.

use crate::cubit::CubitSignal;

pub trait Storage<S> {
    fn read(&self, key: &str) -> Option<S>;
    fn write(&self, key: &str, state: &S) -> bool;
    fn delete(&self, key: &str) -> bool;
}

pub struct HydratedCubit<'a, S: Clone, ST: Storage<S>> {
    cubit: CubitSignal<S>,
    storage: &'a ST,
    storage_key: &'static str,
}

impl<'a, S: Clone + core::fmt::Debug, ST: Storage<S>> HydratedCubit<'a, S, ST> {
    pub fn new(name: &'static str, initial_state: S, storage: &'a ST, storage_key: &'static str) -> Self {
        let loaded_state = storage.read(storage_key).unwrap_or(initial_state);
        let cubit = CubitSignal::new(name, loaded_state);

        Self {
            cubit,
            storage,
            storage_key,
        }
    }

    pub fn cubit(&self) -> &CubitSignal<S> {
        &self.cubit
    }

    pub fn state_value(&self) -> S {
        self.cubit.state_value()
    }

    pub fn emit(&self, new_state: S) -> bool {
        if self.cubit.emit(new_state.clone()) {
            self.storage.write(self.storage_key, &new_state);
            true
        } else {
            false
        }
    }

    pub fn clear_storage(&self) -> bool {
        self.storage.delete(self.storage_key)
    }
}
