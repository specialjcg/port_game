// Domain Events - Event Sourcing foundation
// All state changes are expressed as immutable events

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::value_objects::{BerthId, CraneId, PlayerId, ShipId};

/// Event metadata for event sourcing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub event_id: Uuid,
    pub aggregate_id: Uuid, // Game session ID
    pub timestamp: DateTime<Utc>,
    pub version: u64, // For optimistic locking
}

impl EventMetadata {
    pub fn new(aggregate_id: Uuid, version: u64) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            aggregate_id,
            timestamp: Utc::now(),
            version,
        }
    }
}

/// All possible domain events in the game
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum DomainEvent {
    // Game lifecycle
    GameStarted {
        metadata: EventMetadata,
        player_id: PlayerId,
        ai_player_id: PlayerId,
        num_berths: usize,
        num_cranes: usize,
    },

    TurnStarted {
        metadata: EventMetadata,
        turn_number: u32,
        current_player: PlayerId,
    },

    TurnEnded {
        metadata: EventMetadata,
        turn_number: u32,
        player: PlayerId,
    },

    GameEnded {
        metadata: EventMetadata,
        winner: Option<PlayerId>,
        player_score: i32,
        ai_score: i32,
    },

    // Ship events
    ShipArrived {
        metadata: EventMetadata,
        ship_id: ShipId,
        container_count: u32,
        arrival_time: f64,
    },

    ShipDocked {
        metadata: EventMetadata,
        ship_id: ShipId,
        berth_id: BerthId,
        player: PlayerId,
        docking_time: f64,
    },

    ShipUndocked {
        metadata: EventMetadata,
        ship_id: ShipId,
        berth_id: BerthId,
        completion_time: f64,
        containers_processed: u32,
    },

    // Crane events
    CraneAssigned {
        metadata: EventMetadata,
        crane_id: CraneId,
        ship_id: ShipId,
        player: PlayerId,
        assignment_time: f64,
    },

    CraneUnassigned {
        metadata: EventMetadata,
        crane_id: CraneId,
        ship_id: ShipId,
        unassignment_time: f64,
    },

    ContainerProcessed {
        metadata: EventMetadata,
        crane_id: CraneId,
        ship_id: ShipId,
        containers_remaining: u32,
    },

    // MCTS AI events (for transparency)
    MCTSSearchStarted {
        metadata: EventMetadata,
        player: PlayerId,
        num_simulations: usize,
    },

    MCTSSearchCompleted {
        metadata: EventMetadata,
        player: PlayerId,
        best_action: String, // JSON of the action
        confidence: f64,
        simulations_performed: usize,
    },
}

impl DomainEvent {
    pub fn metadata(&self) -> &EventMetadata {
        match self {
            DomainEvent::GameStarted { metadata, .. } => metadata,
            DomainEvent::TurnStarted { metadata, .. } => metadata,
            DomainEvent::TurnEnded { metadata, .. } => metadata,
            DomainEvent::GameEnded { metadata, .. } => metadata,
            DomainEvent::ShipArrived { metadata, .. } => metadata,
            DomainEvent::ShipDocked { metadata, .. } => metadata,
            DomainEvent::ShipUndocked { metadata, .. } => metadata,
            DomainEvent::CraneAssigned { metadata, .. } => metadata,
            DomainEvent::CraneUnassigned { metadata, .. } => metadata,
            DomainEvent::ContainerProcessed { metadata, .. } => metadata,
            DomainEvent::MCTSSearchStarted { metadata, .. } => metadata,
            DomainEvent::MCTSSearchCompleted { metadata, .. } => metadata,
        }
    }

    pub fn event_type(&self) -> &str {
        match self {
            DomainEvent::GameStarted { .. } => "GameStarted",
            DomainEvent::TurnStarted { .. } => "TurnStarted",
            DomainEvent::TurnEnded { .. } => "TurnEnded",
            DomainEvent::GameEnded { .. } => "GameEnded",
            DomainEvent::ShipArrived { .. } => "ShipArrived",
            DomainEvent::ShipDocked { .. } => "ShipDocked",
            DomainEvent::ShipUndocked { .. } => "ShipUndocked",
            DomainEvent::CraneAssigned { .. } => "CraneAssigned",
            DomainEvent::CraneUnassigned { .. } => "CraneUnassigned",
            DomainEvent::ContainerProcessed { .. } => "ContainerProcessed",
            DomainEvent::MCTSSearchStarted { .. } => "MCTSSearchStarted",
            DomainEvent::MCTSSearchCompleted { .. } => "MCTSSearchCompleted",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_metadata_creation() {
        let aggregate_id = Uuid::new_v4();
        let metadata = EventMetadata::new(aggregate_id, 1);

        assert_eq!(metadata.aggregate_id, aggregate_id);
        assert_eq!(metadata.version, 1);
        assert!(metadata.timestamp <= Utc::now());
    }

    #[test]
    fn test_event_serialization() {
        let metadata = EventMetadata::new(Uuid::new_v4(), 1);
        let event = DomainEvent::ShipArrived {
            metadata: metadata.clone(),
            ship_id: ShipId::new(1),
            container_count: 50,
            arrival_time: 0.0,
        };

        let json = serde_json::to_string(&event).unwrap();
        let deserialized: DomainEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event.event_type(), deserialized.event_type());
    }
}
