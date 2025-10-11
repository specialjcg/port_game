// Commands - Write operations (CQRS pattern)
// Commands express player intentions and generate domain events

use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{BerthId, CraneId, PlayerId, ShipId};

/// All commands that players can issue
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Command {
    /// Dock a waiting ship to a free berth
    DockShip {
        player_id: PlayerId,
        ship_id: ShipId,
        berth_id: BerthId,
    },

    /// Assign a free crane to a docked ship
    AssignCrane {
        player_id: PlayerId,
        crane_id: CraneId,
        ship_id: ShipId,
    },

    /// Unassign a crane from a ship
    UnassignCrane {
        player_id: PlayerId,
        crane_id: CraneId,
        ship_id: ShipId,
    },

    /// Force undock a ship (emergency, penalty applied)
    ForceUndock {
        player_id: PlayerId,
        ship_id: ShipId,
    },

    /// End player's turn (turn-based mode)
    EndTurn { player_id: PlayerId },

    /// AI takes its turn (MCTS decision)
    AITakeTurn {
        player_id: PlayerId,
        num_simulations: usize,
    },
}

impl Command {
    pub fn player_id(&self) -> PlayerId {
        match self {
            Command::DockShip { player_id, .. } => *player_id,
            Command::AssignCrane { player_id, .. } => *player_id,
            Command::UnassignCrane { player_id, .. } => *player_id,
            Command::ForceUndock { player_id, .. } => *player_id,
            Command::EndTurn { player_id } => *player_id,
            Command::AITakeTurn { player_id, .. } => *player_id,
        }
    }

    pub fn command_type(&self) -> &str {
        match self {
            Command::DockShip { .. } => "DockShip",
            Command::AssignCrane { .. } => "AssignCrane",
            Command::UnassignCrane { .. } => "UnassignCrane",
            Command::ForceUndock { .. } => "ForceUndock",
            Command::EndTurn { .. } => "EndTurn",
            Command::AITakeTurn { .. } => "AITakeTurn",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let player_id = PlayerId::new();
        let cmd = Command::DockShip {
            player_id,
            ship_id: ShipId::new(1),
            berth_id: BerthId::new(0),
        };

        assert_eq!(cmd.player_id(), player_id);
        assert_eq!(cmd.command_type(), "DockShip");
    }

    #[test]
    fn test_command_serialization() {
        let cmd = Command::AssignCrane {
            player_id: PlayerId::new(),
            crane_id: CraneId::new(0),
            ship_id: ShipId::new(1),
        };

        let json = serde_json::to_string(&cmd).unwrap();
        let deserialized: Command = serde_json::from_str(&json).unwrap();

        assert_eq!(cmd.command_type(), deserialized.command_type());
    }
}
