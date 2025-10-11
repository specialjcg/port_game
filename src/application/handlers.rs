// Command and Query Handlers
// This is where CQRS commands get translated into domain events

use uuid::Uuid;

use crate::domain::aggregates::Port;
use crate::domain::events::{DomainEvent, EventMetadata};

use super::queries::{PortStateView, ShipView, BerthView, CraneView};

pub struct CommandHandler {
    // Will be implemented when we have full game session
}

pub struct QueryHandler {
    // Will be implemented when we have read models
}

// Temporary: Direct command to event conversion for MVP
pub fn handle_dock_ship_command(
    port: &Port,
    aggregate_id: Uuid,
    ship_id: crate::domain::value_objects::ShipId,
    berth_id: crate::domain::value_objects::BerthId,
    player_id: crate::domain::value_objects::PlayerId,
) -> Result<Vec<DomainEvent>, String> {
    // Validation
    if port.ships.get(&ship_id).is_none() {
        return Err(format!("Ship {} not found", ship_id));
    }

    if !port.berths.get(&berth_id).ok_or("Berth not found")?.is_free() {
        return Err(format!("Berth {} is occupied", berth_id));
    }

    // Generate event
    let event = DomainEvent::ShipDocked {
        metadata: EventMetadata::new(aggregate_id, port.version() + 1),
        ship_id,
        berth_id,
        player: player_id,
        docking_time: port.current_time,
    };

    Ok(vec![event])
}

pub fn handle_assign_crane_command(
    port: &Port,
    aggregate_id: Uuid,
    crane_id: crate::domain::value_objects::CraneId,
    ship_id: crate::domain::value_objects::ShipId,
    player_id: crate::domain::value_objects::PlayerId,
) -> Result<Vec<DomainEvent>, String> {
    // Validation
    if port.ships.get(&ship_id).is_none() {
        return Err(format!("Ship {} not found", ship_id));
    }

    if !port.cranes.get(&crane_id).ok_or("Crane not found")?.is_free() {
        return Err(format!("Crane {} is already assigned", crane_id));
    }

    // Generate event
    let event = DomainEvent::CraneAssigned {
        metadata: EventMetadata::new(aggregate_id, port.version() + 1),
        crane_id,
        ship_id,
        player: player_id,
        assignment_time: port.current_time,
    };

    Ok(vec![event])
}

pub fn query_port_state(port: &Port) -> PortStateView {
    PortStateView {
        player_id: port.player_id,
        ships: port.ships.values().map(ShipView::from).collect(),
        berths: port.berths.values().map(BerthView::from).collect(),
        cranes: port.cranes.values().map(CraneView::from).collect(),
        score: port.calculate_score(),
        current_time: port.current_time,
    }
}
