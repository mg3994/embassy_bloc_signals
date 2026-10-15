use bloc_signals::{BlocObserver, BlocSignal, CubitSignal};
use embassy_executor::Spawner;
use embassy_futures::yield_now;

#[derive(Debug, Clone, PartialEq)]
pub enum CounterEvent {
    Increment,
    Decrement,
    Reset,
}

pub struct CustomObserver;
impl BlocObserver for CustomObserver {
    fn on_event(&self, bloc_name: &'static str, event: &dyn core::fmt::Debug) {
        println!("[{}] Event received: {:?}", bloc_name, event);
    }

    fn on_change(&self, bloc_name: &'static str, change: &dyn core::fmt::Debug) {
        println!("[{}] State changed: {:?}", bloc_name, change);
    }

    fn on_transition(&self, bloc_name: &'static str, transition: &dyn core::fmt::Debug) {
        println!("[{}] State transitioned: {:?}", bloc_name, transition);
    }
}

static OBSERVER: CustomObserver = CustomObserver;

// A Cubit for managing light switch state in an embedded task
pub struct LightCubit {
    cubit: CubitSignal<bool>,
}

impl LightCubit {
    pub fn new() -> Self {
        Self {
            cubit: CubitSignal::new("LightCubit", false),
        }
    }

    pub fn toggle(&self) {
        let current = self.cubit.state_value();
        self.cubit.emit(!current);
    }

    pub fn is_on(&self) -> bool {
        self.cubit.state_value()
    }
}

#[embassy_executor::task]
async fn bloc_task(bloc: &'static BlocSignal<CounterEvent, i32, 16>) {
    bloc.handle_events(|event, current_state| async move {
        yield_now().await;
        match event {
            CounterEvent::Increment => Some(current_state + 1),
            CounterEvent::Decrement => Some(current_state - 1),
            CounterEvent::Reset => Some(0),
        }
    })
    .await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    bloc_signals::set_observer(&OBSERVER);

    println!("--- Testing Embedded Cubit ---");
    let light_cubit = LightCubit::new();
    println!("Initial light state: {}", light_cubit.is_on());

    light_cubit.toggle();
    println!("Light state after toggle: {}", light_cubit.is_on());

    light_cubit.toggle();
    println!("Light state after toggle: {}", light_cubit.is_on());

    println!("\n--- Testing Embedded Async Bloc with Embassy ---");
    static BLOC: BlocSignal<CounterEvent, i32, 16> = BlocSignal::new("CounterBloc", 0);

    spawner.spawn(bloc_task(&BLOC)).unwrap();

    BLOC.add(CounterEvent::Increment);
    BLOC.add(CounterEvent::Increment);
    BLOC.add(CounterEvent::Decrement);

    yield_now().await;
    yield_now().await;
    yield_now().await;

    println!("Final CounterBloc state: {}", BLOC.state_value());
}
