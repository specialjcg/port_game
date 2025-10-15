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
    port.ships
        .insert(ShipId::new(1), Ship::new(ShipId::new(1), 30, 0.0));
    port.ships
        .insert(ShipId::new(2), Ship::new(ShipId::new(2), 40, 1.0));

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
    port.ships
        .insert(ShipId::new(1), Ship::new(ShipId::new(1), 30, 0.0));
    port.ships
        .insert(ShipId::new(2), Ship::new(ShipId::new(2), 40, 1.0));

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
    port.ships
        .insert(ShipId::new(1), Ship::new(ShipId::new(1), 30, 0.0));
    port.ships
        .insert(ShipId::new(2), Ship::new(ShipId::new(2), 40, 1.0));

    // Query state
    let view = query_port_state(&port);

    assert_eq!(view.ships.len(), 2);
    assert_eq!(view.berths.len(), 2);
    assert_eq!(view.cranes.len(), 2);
    assert_eq!(view.player_id, player_id);
}

mod tests {
    use port_game::domain::value_objects::{BerthId, CraneId, PlayerId, ShipId};
    use port_game::game::{EventGenerator, GameMode, GameSession};

    #[test]
    fn test_free_completed_ships() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);
        session.event_generator = EventGenerator::new(0.0);

        // 1. Spawn and dock a ship
        session.spawn_ships(1);
        let ship_id = ShipId::new(0);
        let berth_id = BerthId::new(0);

        // 2. Dock it and assign a crane
        session.player_dock_ship(ship_id, berth_id).unwrap();
        session
            .player_assign_crane(CraneId::new(0), ship_id)
            .unwrap();

        // 3. Empty its containers
        {
            let ship = session.player_port.ships.get_mut(&ship_id).unwrap();
            ship.containers_remaining = 0;
        }

        // 4. Free completed ships
        session.free_completed_ships();

        // 5. Verify the cleanup
        assert!(
            !session.player_port.ships.contains_key(&ship_id),
            "Ship should be removed"
        );
        assert!(
            session.player_port.berths.get(&berth_id).unwrap().is_free(),
            "Berth should be free"
        );

        let crane = session.player_port.cranes.get(&CraneId::new(0)).unwrap();
        assert!(crane.is_free(), "Crane should be unassigned");

        // 6. Verify game state consistency
        assert_eq!(
            session.player_port.docked_ships().len(),
            0,
            "No ships should be docked"
        );
        assert_eq!(
            session.player_port.free_berths().len(),
            2,
            "All berths should be free"
        );
        assert_eq!(
            session.player_port.free_cranes().len(),
            2,
            "All cranes should be free"
        );
    }

    #[test]
    fn test_score_persists_after_ship_completion() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);
        session.event_generator = EventGenerator::new(0.0);

        session.spawn_ships(1);

        let ship_id = ShipId::new(0);
        let berth_id = BerthId::new(0);
        let crane_id = CraneId::new(0);

        session.player_dock_ship(ship_id, berth_id).unwrap();
        session.player_assign_crane(crane_id, ship_id).unwrap();

        // Process enough times to unload the ship completely.
        session.process_containers();
        session.process_containers();

        {
            let ship = session.player_port.ships.get(&ship_id).unwrap();
            assert_eq!(ship.containers_remaining, 0);
        }

        let score_before = session.player_port.calculate_score();
        assert!(
            score_before > 0,
            "processing containers should award points"
        );

        session.free_completed_ships();

        let score_after = session.player_port.calculate_score();
        assert_eq!(
            score_before, score_after,
            "freeing ships should not reset score"
        );
    }

    #[test]
    fn test_complete_turn_cycle() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);
        session.event_generator = EventGenerator::new(0.0);

        // 1. Initial state
        let initial_turn = session.current_turn;
        let initial_score = session.player_port.score;

        // 2. Spawn and dock a ship with cargo
        session.spawn_ships(1);
        let ship_id = ShipId::new(0);
        let berth_id = BerthId::new(0);
        session.player_dock_ship(ship_id, berth_id).unwrap();
        session
            .player_assign_crane(CraneId::new(0), ship_id)
            .unwrap();

        // Sauvegarder l'état initial du navire
        let initial_containers = session
            .player_port
            .ships
            .get(&ship_id)
            .unwrap()
            .containers_remaining;

        // 3. End turn et vérifications
        session.end_turn();

        // 4. Vérifier que le tour a bien avancé
        assert_eq!(
            session.current_turn,
            initial_turn + 1,
            "Le tour devrait être incrémenté"
        );

        // 5. Vérifier le traitement des conteneurs
        let ship = session.player_port.ships.get(&ship_id).unwrap();
        assert!(
            ship.containers_remaining < initial_containers,
            "Les conteneurs du navire devraient avoir diminué"
        );

        // 6. Vérifier les points
        assert!(
            session.player_port.score > initial_score,
            "Le score du joueur devrait avoir augmenté après le traitement des conteneurs"
        );

        // 7. Vérifier l'état des ressources
        let crane = session.player_port.cranes.get(&CraneId::new(0)).unwrap();
        assert!(
            crane.assigned_to.is_some(),
            "La grue devrait rester assignée au navire non complété"
        );
        assert_eq!(
            crane.assigned_to,
            Some(ship_id),
            "La grue devrait toujours être assignée au même navire"
        );

        // 8. Vérifier l'état du quai
        let berth = session.player_port.berths.get(&berth_id).unwrap();
        assert!(!berth.is_free(), "Le quai devrait toujours être occupé");

        // 9. Effectuer un autre tour pour vérifier la continuité
        session.end_turn();
        assert_eq!(
            session.current_turn,
            initial_turn + 2,
            "Le deuxième tour devrait être correctement incrémenté"
        );
    }
}
