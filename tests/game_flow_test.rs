// End-to-end game flow tests
// Simulates a complete game with player actions

use port_game::application::handlers::{
    handle_assign_crane_command, handle_dock_ship_command, query_port_state,
};
use port_game::domain::aggregates::Port;
use port_game::domain::entities::Ship;
use port_game::domain::value_objects::{BerthId, CraneId, PlayerId, ShipId};
use uuid::Uuid;

#[test]
fn test_complete_ship_workflow() {
    // Setup
    let player_id = PlayerId::new();
    let mut port = Port::new(player_id, 2, 2);
    let aggregate_id = Uuid::new_v4();

    // Ship arrives
    let ship_id = ShipId::new(1);
    let ship = Ship::new(ship_id, 50, 0.0);
    port.ships.insert(ship_id, ship);

    // Dock the ship
    let berth_id = BerthId::new(0);
    let events =
        handle_dock_ship_command(&port, aggregate_id, ship_id, berth_id, player_id).unwrap();

    assert_eq!(events.len(), 1);

    for event in &events {
        port.apply_event(event);
    }

    // Verify ship is docked
    let ship = port.ships.get(&ship_id).unwrap();
    assert!(ship.is_docked());
    assert_eq!(ship.docked_at, Some(berth_id));

    // Assign crane
    let crane_id = CraneId::new(0);
    let events =
        handle_assign_crane_command(&port, aggregate_id, crane_id, ship_id, player_id).unwrap();

    for event in &events {
        port.apply_event(event);
    }

    // Verify crane is assigned
    let crane = port.cranes.get(&crane_id).unwrap();
    assert!(!crane.is_free());
    assert_eq!(crane.assigned_to, Some(ship_id));

    let ship = port.ships.get(&ship_id).unwrap();
    assert!(ship.assigned_cranes.contains(&crane_id));
}

#[test]
fn test_multiple_ships_docking() {
    let player_id = PlayerId::new();
    let mut port = Port::new(player_id, 2, 2);
    let aggregate_id = Uuid::new_v4();

    // Add 2 ships
    let ship1 = Ship::new(ShipId::new(1), 30, 0.0);
    let ship2 = Ship::new(ShipId::new(2), 40, 1.0);

    port.ships.insert(ShipId::new(1), ship1);
    port.ships.insert(ShipId::new(2), ship2);

    // Dock first ship
    let events = handle_dock_ship_command(
        &port,
        aggregate_id,
        ShipId::new(1),
        BerthId::new(0),
        player_id,
    )
    .unwrap();
    for event in &events {
        port.apply_event(event);
    }

    // Dock second ship
    let events = handle_dock_ship_command(
        &port,
        aggregate_id,
        ShipId::new(2),
        BerthId::new(1),
        player_id,
    )
    .unwrap();
    for event in &events {
        port.apply_event(event);
    }

    // Verify both are docked
    assert_eq!(port.docked_ships().len(), 2);
    assert_eq!(port.free_berths().len(), 0);
}

#[test]
fn test_cannot_dock_to_occupied_berth() {
    let player_id = PlayerId::new();
    let mut port = Port::new(player_id, 2, 2);
    let aggregate_id = Uuid::new_v4();

    // Add 2 ships
    port.ships.insert(ShipId::new(1), Ship::new(ShipId::new(1), 30, 0.0));
    port.ships.insert(ShipId::new(2), Ship::new(ShipId::new(2), 40, 1.0));

    // Dock first ship to berth 0
    let events = handle_dock_ship_command(
        &port,
        aggregate_id,
        ShipId::new(1),
        BerthId::new(0),
        player_id,
    )
    .unwrap();
    for event in &events {
        port.apply_event(event);
    }

    // Try to dock second ship to same berth - should fail
    let result = handle_dock_ship_command(
        &port,
        aggregate_id,
        ShipId::new(2),
        BerthId::new(0),
        player_id,
    );

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("occupied"));
}

#[test]
fn test_cannot_assign_busy_crane() {
    let player_id = PlayerId::new();
    let mut port = Port::new(player_id, 2, 2);
    let aggregate_id = Uuid::new_v4();

    // Add 2 ships and dock them
    port.ships.insert(ShipId::new(1), Ship::new(ShipId::new(1), 30, 0.0));
    port.ships.insert(ShipId::new(2), Ship::new(ShipId::new(2), 40, 1.0));

    let events = handle_dock_ship_command(
        &port,
        aggregate_id,
        ShipId::new(1),
        BerthId::new(0),
        player_id,
    )
    .unwrap();
    for event in &events {
        port.apply_event(event);
    }

    let events = handle_dock_ship_command(
        &port,
        aggregate_id,
        ShipId::new(2),
        BerthId::new(1),
        player_id,
    )
    .unwrap();
    for event in &events {
        port.apply_event(event);
    }

    // Assign crane 0 to ship 1
    let events = handle_assign_crane_command(
        &port,
        aggregate_id,
        CraneId::new(0),
        ShipId::new(1),
        player_id,
    )
    .unwrap();
    for event in &events {
        port.apply_event(event);
    }

    // Try to assign same crane to ship 2 - should fail
    let result = handle_assign_crane_command(
        &port,
        aggregate_id,
        CraneId::new(0),
        ShipId::new(2),
        player_id,
    );

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already assigned"));
}

#[test]
fn test_query_port_state() {
    let player_id = PlayerId::new();
    let mut port = Port::new(player_id, 2, 2);

    // Add ships
    port.ships.insert(ShipId::new(1), Ship::new(ShipId::new(1), 30, 0.0));
    port.ships.insert(ShipId::new(2), Ship::new(ShipId::new(2), 40, 1.0));

    // Query state
    let view = query_port_state(&port);

    assert_eq!(view.ships.len(), 2);
    assert_eq!(view.berths.len(), 2);
    assert_eq!(view.cranes.len(), 2);
    assert_eq!(view.player_id, player_id);
}
