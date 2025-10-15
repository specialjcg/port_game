// CQRS pattern tests - Commands and Queries

use port_game::application::commands::Command;
use port_game::application::handlers::*;
use port_game::domain::aggregates::Port;
use port_game::domain::entities::Ship;
use port_game::domain::value_objects::*;
use uuid::Uuid;

#[test]
fn test_command_serialization() {
    let player_id = PlayerId::new();

    let commands = vec![
        Command::DockShip {
            player_id,
            ship_id: ShipId::new(1),
            berth_id: BerthId::new(0),
        },
        Command::AssignCrane {
            player_id,
            crane_id: CraneId::new(0),
            ship_id: ShipId::new(1),
        },
        Command::EndTurn { player_id },
        Command::AITakeTurn {
            player_id,
            num_simulations: 100,
        },
    ];

    for cmd in commands {
        // Test JSON serialization
        let json = serde_json::to_string(&cmd).unwrap();
        let deserialized: Command = serde_json::from_str(&json).unwrap();

        assert_eq!(cmd.command_type(), deserialized.command_type());
        assert_eq!(cmd.player_id(), deserialized.player_id());
    }
}

#[test]
fn test_dock_ship_command_validation() {
    let player_id = PlayerId::new();
    let mut port = Port::new(player_id, 2, 2);
    let aggregate_id = Uuid::new_v4();

    // Add a ship
    port.ships
        .insert(ShipId::new(1), Ship::new(ShipId::new(1), 30, 0.0));

    // Valid dock command
    let result = handle_dock_ship_command(
        &port,
        aggregate_id,
        ShipId::new(1),
        BerthId::new(0),
        player_id,
    );
    assert!(result.is_ok());

    // Invalid: ship doesn't exist
    let result = handle_dock_ship_command(
        &port,
        aggregate_id,
        ShipId::new(999),
        BerthId::new(0),
        player_id,
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[test]
fn test_assign_crane_command_validation() {
    let player_id = PlayerId::new();
    let mut port = Port::new(player_id, 2, 2);
    let aggregate_id = Uuid::new_v4();

    // Setup: Add and dock a ship
    port.ships
        .insert(ShipId::new(1), Ship::new(ShipId::new(1), 30, 0.0));

    let dock_events = handle_dock_ship_command(
        &port,
        aggregate_id,
        ShipId::new(1),
        BerthId::new(0),
        player_id,
    )
    .unwrap();

    for event in &dock_events {
        port.apply_event(event);
    }

    // Valid assign crane command
    let result = handle_assign_crane_command(
        &port,
        aggregate_id,
        CraneId::new(0),
        ShipId::new(1),
        player_id,
    );
    assert!(result.is_ok());

    // Apply the event
    for event in &result.unwrap() {
        port.apply_event(event);
    }

    // Invalid: crane already assigned
    let result = handle_assign_crane_command(
        &port,
        aggregate_id,
        CraneId::new(0),
        ShipId::new(1),
        player_id,
    );
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already assigned"));
}

#[test]
fn test_query_port_state() {
    let player_id = PlayerId::new();
    let mut port = Port::new(player_id, 2, 2);

    // Add two ships
    let ship1 = Ship::new(ShipId::new(1), 30, 0.0);
    let ship2 = Ship::new(ShipId::new(2), 30, 1.0); // Changé de 40 à 30 pour la cohérence

    port.ships.insert(ShipId::new(1), ship1);
    port.ships.insert(ShipId::new(2), ship2);

    // Query state
    let view = query_port_state(&port);

    // Verify view state
    assert_eq!(view.ships.len(), 2);
    for ship in &view.ships {
        assert_eq!(ship.containers_remaining, 30);
    }
    assert_eq!(view.berths.len(), 2);
    assert_eq!(view.cranes.len(), 2);
    assert_eq!(view.player_id, player_id);
}

#[test]
fn test_event_sourcing_command_flow() {
    let player_id = PlayerId::new();
    let mut port = Port::new(player_id, 2, 2);
    let aggregate_id = Uuid::new_v4();

    let initial_version = port.version();

    // Add ship
    port.ships
        .insert(ShipId::new(1), Ship::new(ShipId::new(1), 50, 0.0));

    // Execute dock command
    let events = handle_dock_ship_command(
        &port,
        aggregate_id,
        ShipId::new(1),
        BerthId::new(0),
        player_id,
    )
    .unwrap();

    assert_eq!(events.len(), 1);

    // Apply events
    for event in &events {
        port.apply_event(event);
    }

    // Version should increment
    assert_eq!(port.version(), initial_version + 1);

    // State should be updated
    let ship = port.ships.get(&ShipId::new(1)).unwrap();
    assert!(ship.is_docked());
    assert_eq!(ship.docked_at, Some(BerthId::new(0)));

    let berth = port.berths.get(&BerthId::new(0)).unwrap();
    assert!(!berth.is_free());
}

#[test]
fn test_command_idempotency() {
    let player_id = PlayerId::new();
    let mut port = Port::new(player_id, 2, 2);
    let aggregate_id = Uuid::new_v4();

    port.ships
        .insert(ShipId::new(1), Ship::new(ShipId::new(1), 30, 0.0));

    // First dock command - should succeed
    let result1 = handle_dock_ship_command(
        &port,
        aggregate_id,
        ShipId::new(1),
        BerthId::new(0),
        player_id,
    );
    assert!(result1.is_ok());

    // Apply events
    for event in &result1.unwrap() {
        port.apply_event(event);
    }

    // Second dock command to same berth - should fail
    let result2 = handle_dock_ship_command(
        &port,
        aggregate_id,
        ShipId::new(1),
        BerthId::new(0),
        player_id,
    );
    assert!(result2.is_err());
}

#[test]
fn test_complex_workflow() {
    let player_id = PlayerId::new();
    let mut port = Port::new(player_id, 2, 2);
    let aggregate_id = Uuid::new_v4();

    // Add two ships
    port.ships
        .insert(ShipId::new(1), Ship::new(ShipId::new(1), 30, 0.0));
    port.ships
        .insert(ShipId::new(2), Ship::new(ShipId::new(2), 30, 1.0));

    // Dock ship 1 to berth 0
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

    // Dock ship 2 to berth 1
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

    // Assign crane 1 to ship 2
    let events = handle_assign_crane_command(
        &port,
        aggregate_id,
        CraneId::new(1),
        ShipId::new(2),
        player_id,
    )
    .unwrap();
    for event in &events {
        port.apply_event(event);
    }

    // Verify final state
    assert_eq!(port.docked_ships().len(), 2);
    assert_eq!(port.free_berths().len(), 0);
    assert_eq!(port.free_cranes().len(), 0);

    let ship1 = port.ships.get(&ShipId::new(1)).unwrap();
    assert_eq!(ship1.assigned_cranes.len(), 1);

    let ship2 = port.ships.get(&ShipId::new(2)).unwrap();
    assert_eq!(ship2.assigned_cranes.len(), 1);
}
