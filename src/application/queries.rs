// Queries - Read operations (CQRS pattern)
// Queries return current state without modifying it

use serde::{Deserialize, Serialize};

use crate::domain::entities::{Berth, Crane, Ship};
use crate::domain::value_objects::PlayerId;

/// All queries for reading game state
#[derive(Debug, Clone)]
pub enum Query {
    /// Get current port state for a player
    GetPortState { player_id: PlayerId },

    /// Get all waiting ships
    GetWaitingShips { player_id: PlayerId },

    /// Get all docked ships
    GetDockedShips { player_id: PlayerId },

    /// Get free berths
    GetFreeBerths { player_id: PlayerId },

    /// Get free cranes
    GetFreeCranes { player_id: PlayerId },

    /// Get current score
    GetScore { player_id: PlayerId },

    /// Get comparison stats (player vs AI)
    GetComparisonStats,

    /// Get MCTS tree state (for visualization)
    GetMCTSState { player_id: PlayerId },
}

/// Query results - View models for read side
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortStateView {
    pub player_id: PlayerId,
    pub ships: Vec<ShipView>,
    pub berths: Vec<BerthView>,
    pub cranes: Vec<CraneView>,
    pub score: i32,
    pub current_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipView {
    pub id: usize,
    pub containers: u32,
    pub containers_remaining: u32,
    pub is_docked: bool,
    pub docked_at: Option<usize>,
    pub assigned_cranes: Vec<usize>,
}

impl From<&Ship> for ShipView {
    fn from(ship: &Ship) -> Self {
        Self {
            id: ship.id.0,
            containers: ship.containers,
            containers_remaining: ship.containers_remaining,
            is_docked: ship.is_docked(),
            docked_at: ship.docked_at.map(|b| b.0),
            assigned_cranes: ship.assigned_cranes.iter().map(|c| c.0).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BerthView {
    pub id: usize,
    pub is_free: bool,
    pub occupied_by: Option<usize>,
}

impl From<&Berth> for BerthView {
    fn from(berth: &Berth) -> Self {
        Self {
            id: berth.id.0,
            is_free: berth.is_free(),
            occupied_by: berth.occupied_by.map(|s| s.0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraneView {
    pub id: usize,
    pub is_free: bool,
    pub assigned_to: Option<usize>,
    pub processing_speed: f64,
}

impl From<&Crane> for CraneView {
    fn from(crane: &Crane) -> Self {
        Self {
            id: crane.id.0,
            is_free: crane.is_free(),
            assigned_to: crane.assigned_to.map(|s| s.0),
            processing_speed: crane.processing_speed,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonStats {
    pub player_score: i32,
    pub ai_score: i32,
    pub player_ships_processed: u32,
    pub ai_ships_processed: u32,
    pub player_avg_wait_time: f64,
    pub ai_avg_wait_time: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Ship;
    use crate::domain::value_objects::ShipId;

    #[test]
    fn test_ship_view_conversion() {
        let ship = Ship::new(ShipId::new(1), 50, 0.0);
        let view = ShipView::from(&ship);

        assert_eq!(view.id, 1);
        assert_eq!(view.containers, 50);
        assert!(!view.is_docked);
    }
}
