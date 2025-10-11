// Port Game - MCTS Strategy Game with CQRS + Event Sourcing
// Architecture: Hexagonal (Ports & Adapters) + CQRS + Event Sourcing

pub mod domain;      // Core business logic (pure, no dependencies)
pub mod application; // Use cases, command/query handlers
pub mod infrastructure; // Event store, persistence, external adapters
pub mod game;        // Game-specific orchestration
pub mod mcts;        // Monte Carlo Tree Search engine

// Re-exports for convenience
pub use domain::events::DomainEvent;
pub use application::commands::Command;
pub use application::queries::Query;
