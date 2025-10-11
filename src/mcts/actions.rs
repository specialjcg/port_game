// MCTS Actions - Possible moves in the game

use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{BerthId, CraneId, ShipId};

/// All possible actions in the game (for MCTS simulation)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MCTSAction {
    /// Dock a ship to a berth
    DockShip {
        ship_id: ShipId,
        berth_id: BerthId,
    },

    /// Assign a crane to a docked ship
    AssignCrane {
        crane_id: CraneId,
        ship_id: ShipId,
    },

    /// Unassign a crane
    UnassignCrane {
        crane_id: CraneId,
    },

    /// Do nothing (pass)
    Pass,
}

impl MCTSAction {
    pub fn action_type(&self) -> &str {
        match self {
            MCTSAction::DockShip { .. } => "DockShip",
            MCTSAction::AssignCrane { .. } => "AssignCrane",
            MCTSAction::UnassignCrane { .. } => "UnassignCrane",
            MCTSAction::Pass => "Pass",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_type() {
        let action = MCTSAction::DockShip {
            ship_id: ShipId::new(1),
            berth_id: BerthId::new(0),
        };

        assert_eq!(action.action_type(), "DockShip");
    }

    #[test]
    fn test_action_equality() {
        let a1 = MCTSAction::Pass;
        let a2 = MCTSAction::Pass;

        assert_eq!(a1, a2);
    }
}
