//! Event concurrency transformers.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventTransformer {
    Sequential,
    Droppable,
    Restartable,
    Concurrent,
}
