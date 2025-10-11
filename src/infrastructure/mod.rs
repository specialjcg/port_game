// Infrastructure layer - Technical concerns (persistence, I/O)

pub mod event_store;

pub use event_store::{EventStore, InMemoryEventStore};
