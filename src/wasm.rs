// WebAssembly bindings for browser integration
// Exposes Rust game logic to JavaScript

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use crate::domain::value_objects::{BerthId, CraneId, PlayerId, ShipId};
#[cfg(feature = "wasm")]
use crate::game::{GameMode, GameSession};

/// JavaScript console logging
#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// WebAssembly game wrapper
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmGame {
    session: GameSession,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmGame {
    /// Create a new game session
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Set panic hook for better error messages
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

        log("Port Game initialized in WebAssembly!");

        Self { session }
    }

    /// Start a new turn
    #[wasm_bindgen(js_name = startTurn)]
    pub fn start_turn(&mut self) {
        self.session.start_turn();
    }

    /// End the current turn
    #[wasm_bindgen(js_name = endTurn)]
    pub fn end_turn(&mut self) {
        self.session.end_turn();
    }

    /// Spawn ships
    #[wasm_bindgen(js_name = spawnShips)]
    pub fn spawn_ships(&mut self, count: usize) {
        self.session.spawn_ships(count);
    }

    /// Player docks a ship
    #[wasm_bindgen(js_name = dockShip)]
    pub fn dock_ship(&mut self, ship_id: usize, berth_id: usize) -> Result<(), JsValue> {
        self.session
            .player_dock_ship(ShipId::new(ship_id), BerthId::new(berth_id))
            .map_err(|e| JsValue::from_str(&e))
    }

    /// Player assigns crane
    #[wasm_bindgen(js_name = assignCrane)]
    pub fn assign_crane(&mut self, crane_id: usize, ship_id: usize) -> Result<(), JsValue> {
        // Conversion des IDs en évitant les allocations inutiles
        let crane = CraneId::new(crane_id);
        let ship = ShipId::new(ship_id);

        // Vérification préalable pour éviter les allocations en cas d'erreur
        if !self.session.player_port.cranes.contains_key(&crane) {
            return Err(JsValue::from_str("Crane not found"));
        }
        if !self.session.player_port.ships.contains_key(&ship) {
            return Err(JsValue::from_str("Ship not found"));
        }

        // Assignation avec gestion d'erreur simplifiée
        match self.session.player_assign_crane(crane, ship) {
            Ok(()) => Ok(()),
            Err(e) => {
                // Conversion d'erreur sans allocation dynamique
                Err(JsValue::from_str(e.as_str()))
            }
        }
    }

    /// Process containers
    #[wasm_bindgen(js_name = "processContainers")]
    pub fn process_containers(&mut self) {
        // Collecter d'abord les informations nécessaires
        let completed_ships: Vec<_> = self
            .session
            .player_port
            .ships
            .iter()
            .filter(|(_, ship)| ship.is_docked() && ship.containers_remaining == 0)
            .map(|(id, ship)| (*id, ship.docked_at.unwrap(), ship.assigned_cranes.clone()))
            .collect();

        // Traiter les conteneurs
        self.session.process_containers();

        // Libérer les navires terminés
        for (ship_id, berth_id, crane_ids) in completed_ships {
            // Libérer les grues
            for crane_id in crane_ids {
                self.session.player_port.free_crane(crane_id);
            }
            // Libérer le navire
            self.session.player_port.undock_ship(ship_id, berth_id);
        }
    }

    /// AI takes turn
    #[wasm_bindgen(js_name = aiTakeTurn)]
    pub fn ai_take_turn(&mut self) {
        self.session.ai_take_turn();
    }

    /// Process random events
    #[wasm_bindgen(js_name = processRandomEvents)]
    pub fn process_random_events(&mut self) -> JsValue {
        let events = self.session.process_random_events();
        let descriptions: Vec<String> = events.iter().map(|e| e.description()).collect();
        serde_wasm_bindgen::to_value(&descriptions).unwrap_or(JsValue::NULL)
    }

    /// Get player port state as JSON
    #[wasm_bindgen(js_name = getPlayerPort)]
    pub fn get_player_port(&self) -> JsValue {
        use crate::application::handlers::query_port_state;
        let view = query_port_state(&self.session.player_port);
        serde_wasm_bindgen::to_value(&view).unwrap_or(JsValue::NULL)
    }

    /// Get AI port state as JSON
    #[wasm_bindgen(js_name = getAiPort)]
    pub fn get_ai_port(&self) -> JsValue {
        use crate::application::handlers::query_port_state;
        let view = query_port_state(&self.session.ai_port);
        serde_wasm_bindgen::to_value(&view).unwrap_or(JsValue::NULL)
    }

    /// Get current turn number
    #[wasm_bindgen(js_name = getCurrentTurn)]
    pub fn get_current_turn(&self) -> u32 {
        self.session.current_turn
    }

    /// Check if game is over
    #[wasm_bindgen(js_name = isGameOver)]
    pub fn is_game_over(&self) -> bool {
        self.session.is_game_over()
    }

    /// Get winner (null if not game over)
    #[wasm_bindgen(js_name = getWinner)]
    pub fn get_winner(&self) -> Option<String> {
        self.session.get_winner().map(|s| s.to_string())
    }

    /// Export replay as JSON
    #[wasm_bindgen(js_name = exportReplay)]
    pub fn export_replay(&self) -> Result<String, JsValue> {
        self.session
            .export_replay()
            .map_err(|e| JsValue::from_str(&e))
    }

    /// Get active effects description
    #[wasm_bindgen(js_name = getActiveEffects)]
    pub fn get_active_effects(&self) -> JsValue {
        let effects = self.session.get_active_effects_description();
        serde_wasm_bindgen::to_value(&effects).unwrap_or(JsValue::NULL)
    }

    /// Get crane efficiency modifier
    #[wasm_bindgen(js_name = getCraneEfficiency)]
    pub fn get_crane_efficiency(&self) -> f64 {
        self.session.crane_efficiency_modifier
    }

    /// Free completed ships and their resources
    #[wasm_bindgen(js_name = freeCompletedShips)]
    pub fn free_completed_ships(&mut self) {
        self.session.free_completed_ships();
    }
}

// Add serde-wasm-bindgen for easier serialization
#[cfg(feature = "wasm")]
use serde_wasm_bindgen;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_feature_flag() {
        // This test verifies the module compiles
        #[cfg(feature = "wasm")]
        {
            // WASM feature is enabled
            assert!(true);
        }

        #[cfg(not(feature = "wasm"))]
        {
            // WASM feature is disabled
            assert!(true);
        }
    }
}
