// Application layer - Use cases, orchestration of domain logic
// CQRS pattern: separates Commands (write) from Queries (read)

pub mod commands;
pub mod handlers;
pub mod queries;

pub use commands::Command;
pub use queries::Query;
