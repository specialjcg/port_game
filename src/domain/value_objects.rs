// Value Objects - Immutable, type-safe identifiers
// Following DDD principles with newtype pattern

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Type-safe ship identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShipId(pub usize);

impl ShipId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

impl fmt::Display for ShipId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ship#{}", self.0)
    }
}

/// Type-safe berth identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BerthId(pub usize);

impl BerthId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

impl fmt::Display for BerthId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Berth#{}", self.0)
    }
}

/// Type-safe crane identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CraneId(pub usize);

impl CraneId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

impl fmt::Display for CraneId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Crane#{}", self.0)
    }
}

/// Type-safe player identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub Uuid);

impl PlayerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player[{}]", &self.0.to_string()[..8])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship_id_creation() {
        let ship_id = ShipId::new(42);
        assert_eq!(ship_id.0, 42);
        assert_eq!(ship_id.to_string(), "Ship#42");
    }

    #[test]
    fn test_berth_id_equality() {
        let b1 = BerthId::new(1);
        let b2 = BerthId::new(1);
        let b3 = BerthId::new(2);

        assert_eq!(b1, b2);
        assert_ne!(b1, b3);
    }

    #[test]
    fn test_player_id_unique() {
        let p1 = PlayerId::new();
        let p2 = PlayerId::new();

        assert_ne!(p1, p2); // Should be different UUIDs
    }
}
