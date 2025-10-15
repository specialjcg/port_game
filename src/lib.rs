// Port Game - MCTS Strategy Game with CQRS + Event Sourcing
// Architecture: Hexagonal (Ports & Adapters) + CQRS + Event Sourcing

pub mod application; // Use cases, command/query handlers
pub mod cli; // Command-line interface
pub mod domain; // Core business logic (pure, no dependencies)
pub mod game; // Game-specific orchestration
pub mod infrastructure; // Event store, persistence, external adapters
pub mod mcts; // Monte Carlo Tree Search engine
pub mod utils; // Shared utilities (e.g., cross-target randomness)

#[cfg(feature = "wasm")]
pub mod wasm; // WebAssembly bindings

// Re-exports for convenience
pub use application::commands::Command;
pub use application::queries::Query;
pub use domain::events::DomainEvent;
