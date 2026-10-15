# bloc_signals

A high-performance, `#![no_std]` capable reactive state management library for Rust and [Embassy](https://embassy.dev/), bridging Business Logic Component (BLoC) architecture with fine-grained reactive Signals primitives.

## Features

- **`#![no_std]` Compatible**: Designed for microcontrollers with zero dynamic memory allocation required by default.
- **`CubitSignal<S>`**: Simple state management container driven by direct synchronous `emit` updates.
- **`BlocSignal<E, S, N>`**: Event-driven state management with async event handlers and Embassy channel event queues.
- **Event Concurrency Transformers**: Built-in support for `sequential`, `droppable`, `restartable`, and `concurrent` event handling.
- **Reactive Signals**: Fine-grained state value holding, custom equality deduplication, and watch channels (`embassy-sync`).
- **`ReplayCubit`**: Undo and redo state history tracking using fixed-capacity `#![no_std]` circular buffers.
- **`HydratedCubit`**: Local state persistence and hydration over flash / NVS storage traits.
- **`ComputedSignal`**: Derived reactive state calculated reactively from source signals.
- **Global Observer**: Global `BlocObserver` telemetry hook for observing event dispatches, state transitions, and lifecycle events.

## License

Dual-licensed under MIT or Apache-2.0.
