// Application layer - Use cases, orchestration of domain logic
// CQRS pattern: separates Commands (write) from Queries (read)

pub mod commands;
pub mod queries;
pub mod handlers;

pub use commands::Command;
pub use queries::Query;
