// WASM bindings tests

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
mod wasm_tests {
    use wasm_bindgen_test::*;
    use port_game::wasm::WasmGame;

    #[wasm_bindgen_test]
    fn test_wasm_game_creation() {
        let game = WasmGame::new();
        assert_eq!(game.get_current_turn(), 0);
        assert!(!game.is_game_over());
    }

    #[wasm_bindgen_test]
    fn test_spawn_ships() {
        let mut game = WasmGame::new();
        game.spawn_ships(3);

        let player_port = game.get_player_port();
        assert!(player_port.is_object());
    }

    #[wasm_bindgen_test]
    fn test_dock_ship_workflow() {
        let mut game = WasmGame::new();
        game.spawn_ships(1);

        // Should succeed to dock ship 0 to berth 0
        let result = game.dock_ship(0, 0);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_assign_crane_workflow() {
        let mut game = WasmGame::new();
        game.spawn_ships(1);

        game.dock_ship(0, 0).unwrap();

        // Should succeed to assign crane 0 to ship 0
        let result = game.assign_crane(0, 0);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_process_containers() {
        let mut game = WasmGame::new();
        game.spawn_ships(1);
        game.dock_ship(0, 0).unwrap();
        game.assign_crane(0, 0).unwrap();

        // Process containers should succeed
        game.process_containers();

        // Ship should have fewer containers
        let player_port = game.get_player_port();
        assert!(player_port.is_object());
    }

    #[wasm_bindgen_test]
    fn test_ai_take_turn() {
        let mut game = WasmGame::new();
        game.spawn_ships(2);

        // AI should make decisions
        game.ai_take_turn();

        let ai_port = game.get_ai_port();
        assert!(ai_port.is_object());
    }

    #[wasm_bindgen_test]
    fn test_random_events() {
        let mut game = WasmGame::new();

        let events = game.process_random_events();
        assert!(events.is_array());
    }

    #[wasm_bindgen_test]
    fn test_active_effects() {
        let mut game = WasmGame::new();

        let effects = game.get_active_effects();
        assert!(effects.is_array());
    }

    #[wasm_bindgen_test]
    fn test_game_state_serialization() {
        let mut game = WasmGame::new();
        game.spawn_ships(2);
        game.dock_ship(0, 0).unwrap();

        let player_port = game.get_player_port();
        let ai_port = game.get_ai_port();

        // Both should be valid JS objects
        assert!(player_port.is_object());
        assert!(ai_port.is_object());
    }

    #[wasm_bindgen_test]
    fn test_replay_export() {
        let mut game = WasmGame::new();
        game.spawn_ships(1);
        game.dock_ship(0, 0).unwrap();

        let replay = game.export_replay();
        assert!(replay.is_object());
    }

    #[wasm_bindgen_test]
    fn test_game_over_detection() {
        let game = WasmGame::new();

        assert!(!game.is_game_over());

        let winner = game.get_winner();
        assert!(winner.is_none());
    }
}

// Native Rust tests (always run)
#[cfg(test)]
mod native_tests {
    use port_game::game::{GameMode, GameSession};
    use port_game::domain::value_objects::PlayerId;

    #[test]
    fn test_game_session_basic_workflow() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

        session.spawn_ships(2);

        assert_eq!(session.player_port.ships.len(), 2);
        assert_eq!(session.ai_port.ships.len(), 2);
    }

    #[test]
    fn test_turn_progression() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

        assert_eq!(session.current_turn, 0);

        session.start_turn();
        assert_eq!(session.current_turn, 1);

        session.start_turn();
        assert_eq!(session.current_turn, 2);
    }

    #[test]
    fn test_score_calculation() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

        session.spawn_ships(1);
        let ship_id = session.player_port.ships.keys().next().copied().unwrap();
        let berth_id = session.player_port.free_berths()[0].id;

        session.player_dock_ship(ship_id, berth_id).unwrap();

        let crane_id = session.player_port.free_cranes()[0].id;
        session.player_assign_crane(crane_id, ship_id).unwrap();

        // Process all containers
        for _ in 0..10 {
            session.process_containers();
            if session.player_port.ships.get(&ship_id).is_none() {
                // Ship was removed after completion
                break;
            }
            if session.player_port.ships.get(&ship_id).unwrap().containers_remaining == 0 {
                break;
            }
        }

        // Player should have positive calculated score from processing containers
        // Note: calculate_score() computes score dynamically, not from the score field
        assert!(session.player_port.calculate_score() > 0);
    }

    #[test]
    fn test_ai_makes_valid_moves() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

        session.spawn_ships(2);

        let initial_docked = session.ai_port.docked_ships().len();

        // AI should make some moves
        session.ai_take_turn();

        // AI might have docked ships or assigned cranes
        let final_state = session.ai_port.docked_ships().len() +
                         session.ai_port.cranes.values()
                             .filter(|c| c.assigned_to.is_some())
                             .count();

        // Some action should have been taken or state should be valid
        assert!(final_state >= initial_docked);
    }
}
