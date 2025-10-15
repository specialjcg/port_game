// Domain layer - Pure business logic, no external dependencies

pub mod aggregates;
pub mod entities;
pub mod events;
pub mod value_objects;

pub use aggregates::Port;
pub use entities::{Berth, Crane, Ship};
pub use value_objects::{BerthId, CraneId, PlayerId, ShipId};
