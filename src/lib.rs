#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod signal;
pub mod cubit;
pub mod bloc;
pub mod transformer;
pub mod observer;

pub use signal::{Signal, ReadonlySignal, SignalOptions, Equality};
pub use cubit::CubitSignal;
pub use bloc::BlocSignal;
pub use transformer::EventTransformer;
pub use observer::{BlocObserver, set_observer, get_observer, Transition, Change};
