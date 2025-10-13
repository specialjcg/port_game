//! Tests d'intégration pour port_game
use port_game::domain::value_objects::{BerthId, CraneId, PlayerId, ShipId};
use port_game::game::{GameMode, GameSession};
use port_game::infrastructure::event_store::{EventStore, InMemoryEventStore}; // Correction du chemin d'import

#[test]
fn test_game_session_initialization() {
    let player_id = PlayerId::new();
    let ai_id = PlayerId::new();

    let session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

    assert_eq!(session.mode, GameMode::VersusAI);
    assert_eq!(session.current_turn, 0);
    assert_eq!(session.player_port.berths.len(), 2);
    assert_eq!(session.player_port.cranes.len(), 2);
}

#[test]
fn test_game_flow_with_ships() {
    let player_id = PlayerId::new();
    let ai_id = PlayerId::new();

    let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

    // Start turn and spawn ships
    session.start_turn();
    session.spawn_ships(2);

    assert_eq!(session.current_turn, 1);
    assert_eq!(session.player_port.ships.len(), 2);
    assert_eq!(session.ai_port.ships.len(), 2);

    // Verify ships are waiting
    assert_eq!(session.player_port.waiting_ships().len(), 2);
    assert_eq!(session.player_port.docked_ships().len(), 0);
}

#[test]
fn test_event_sourcing_replay() {
    let player_id = PlayerId::new();
    let ai_id = PlayerId::new();

    let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);
    session.start_turn();
    session.spawn_ships(3);

    // Export events
    let replay_json = session.export_replay().expect("Failed to export replay");

    // Verify JSON is valid and contains events
    assert!(replay_json.contains("GameStarted"));
    assert!(replay_json.contains("TurnStarted"));
    assert!(replay_json.contains("ShipArrived"));

    // Parse JSON to verify it's valid
    let events: Vec<port_game::domain::events::DomainEvent> =
        serde_json::from_str(&replay_json).expect("Failed to parse replay JSON");

    // Should have: 1 GameStarted + 1 TurnStarted + 3 ShipArrived = 5 events
    assert!(events.len() >= 5);
}

#[test]
fn test_multiple_turns() {
    let player_id = PlayerId::new();
    let ai_id = PlayerId::new();

    let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

    // Turn 1
    session.start_turn();
    assert_eq!(session.current_turn, 1);

    // Turn 2
    session.start_turn();
    assert_eq!(session.current_turn, 2);

    // Turn 3
    session.start_turn();
    assert_eq!(session.current_turn, 3);
}

#[test]
fn test_port_state_queries() {
    let player_id = PlayerId::new();
    let ai_id = PlayerId::new();

    let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);
    session.spawn_ships(2);

    let player_port = session.get_player_port();

    // Check initial state
    assert_eq!(player_port.ships.len(), 2);
    assert_eq!(player_port.free_berths().len(), 2);
    assert_eq!(player_port.free_cranes().len(), 2);

    // Calculate initial score (should be negative due to waiting time)
    let score = player_port.calculate_score();
    assert!(score <= 0); // Negative or zero due to waiting penalty
}

#[test]
fn test_event_store_isolation() {
    let player_id = PlayerId::new();
    let ai_id = PlayerId::new();

    let session1 = GameSession::new(GameMode::VersusAI, player_id, ai_id);
    let session2 = GameSession::new(GameMode::VersusAI, player_id, ai_id);

    // Different sessions should have different IDs
    assert_ne!(session1.session_id, session2.session_id);

    // Events should be isolated
    let replay1 = session1.export_replay().unwrap();
    let replay2 = session2.export_replay().unwrap();

    // Both should have GameStarted events
    assert!(replay1.contains("GameStarted"));
    assert!(replay2.contains("GameStarted"));
}
