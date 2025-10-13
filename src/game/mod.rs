// Game orchestration layer - High-level game logic

pub mod events;

use uuid::Uuid;

use crate::domain::aggregates::Port;
use crate::domain::events::{DomainEvent, EventMetadata};
use crate::domain::value_objects::{BerthId, CraneId, PlayerId, ShipId};
use crate::infrastructure::{EventStore, InMemoryEventStore};
use crate::mcts::{MCTSConfig, MCTSEngine};

pub use events::{ActiveEvent, EventGenerator, RandomEvent};

/// Game mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    VersusAI,  // Player vs AI MCTS
    Tutorial,  // Learning mode
    Sandbox,   // Free play
}

/// Game session - Main game state manager
pub struct GameSession {
    pub session_id: Uuid,
    pub mode: GameMode,
    pub player_port: Port,
    pub ai_port: Port,
    pub current_turn: u32,
    pub current_player: PlayerId,
    pub mcts_engine: MCTSEngine,
    pub event_store: InMemoryEventStore,
    pub event_generator: EventGenerator,
    pub active_events: Vec<ActiveEvent>,
    pub crane_efficiency_modifier: f64, // 1.0 = normal, <1.0 = penalty, >1.0 = bonus
}

impl GameSession {
    pub fn new(mode: GameMode, player_id: PlayerId, ai_id: PlayerId) -> Self {
        let session_id = Uuid::new_v4();

        // Simple configuration: 2 berths, 2 cranes
        let player_port = Port::new(player_id, 2, 2);
        let ai_port = Port::new(ai_id, 2, 2);

        let mcts_config = MCTSConfig {
            num_simulations: 100, // Small for MVP
            exploration_constant: 1.41,
            max_depth: 20,
        };

        let mcts_engine = MCTSEngine::new(mcts_config);
        let mut event_store = InMemoryEventStore::new();

        // Emit GameStarted event
        let start_event = DomainEvent::GameStarted {
            metadata: EventMetadata::new(session_id, 1),
            player_id,
            ai_player_id: ai_id,
            num_berths: 2,
            num_cranes: 2,
        };

        event_store.append(session_id, vec![start_event]).ok();

        Self {
            session_id,
            mode,
            player_port,
            ai_port,
            current_turn: 0,
            current_player: player_id,
            mcts_engine,
            event_store,
            event_generator: EventGenerator::default(),
            active_events: Vec::new(),
            crane_efficiency_modifier: 1.0,
        }
    }

    pub fn start_turn(&mut self) {
        self.current_turn += 1;

        // Nous ne libérons plus automatiquement toutes les grues
        // self.player_port.free_all_cranes();
        // self.ai_port.free_all_cranes();

        let event = DomainEvent::TurnStarted {
            metadata: EventMetadata::new(self.session_id, self.current_turn as u64),
            turn_number: self.current_turn,
            current_player: self.current_player,
        };

        self.event_store.append(self.session_id, vec![event]).ok();
    }

    pub fn spawn_ships(&mut self, count: usize) {
        let mut events = Vec::new();

        for i in 0..count {
            let ship_id = ShipId::new(self.current_turn as usize * 10 + i);
            let containers = 20 + (i * 10) as u32; // Varying sizes

            let event = DomainEvent::ShipArrived {
                metadata: EventMetadata::new(self.session_id, self.player_port.version() + 1),
                ship_id,
                container_count: containers,
                arrival_time: self.current_turn as f64,
            };

            events.push(event.clone());
            self.player_port.apply_event(&event);
            self.ai_port.apply_event(&event);
        }

        self.event_store.append(self.session_id, events).ok();
    }

    pub fn get_player_port(&self) -> &Port {
        &self.player_port
    }

    pub fn get_ai_port(&self) -> &Port {
        &self.ai_port
    }

    pub fn export_replay(&self) -> Result<String, String> {
        self.event_store.export_to_json(self.session_id)
    }

    /// Player docks a ship
    pub fn player_dock_ship(
        &mut self,
        ship_id: ShipId,
        berth_id: crate::domain::value_objects::BerthId,
    ) -> Result<(), String> {
        use crate::application::handlers::handle_dock_ship_command;

        let events = handle_dock_ship_command(
            &self.player_port,
            self.session_id,
            ship_id,
            berth_id,
            self.player_port.player_id,
        )?;

        for event in &events {
            self.player_port.apply_event(event);
        }

        self.event_store.append(self.session_id, events).ok();
        Ok(())
    }

    /// Player assigns crane
    pub fn player_assign_crane(
        &mut self,
        crane_id: crate::domain::value_objects::CraneId,
        ship_id: ShipId,
    ) -> Result<(), String> {
        use crate::application::handlers::handle_assign_crane_command;

        let events = handle_assign_crane_command(
            &self.player_port,
            self.session_id,
            crane_id,
            ship_id,
            self.player_port.player_id,
        )?;

        for event in &events {
            self.player_port.apply_event(event);
        }

        self.event_store.append(self.session_id, events).ok();
        Ok(())
    }

    /// Process containers for all docked ships with assigned cranes
    pub fn process_containers(&mut self) {
        use crate::domain::events::DomainEvent;

        // Player port
        let mut events = Vec::new();
        for ship in self.player_port.docked_ships() {
            if !ship.assigned_cranes.is_empty() {
                let crane_count = ship.assigned_cranes.len() as u32;
                let base_amount = crane_count * 10; // Each crane processes 10 containers
                let process_amount = (base_amount as f64 * self.crane_efficiency_modifier) as u32;

                if ship.containers_remaining > 0 {
                    let processed = process_amount.min(ship.containers_remaining);
                    let remaining = ship.containers_remaining - processed;

                    let event = DomainEvent::ContainerProcessed {
                        metadata: EventMetadata::new(self.session_id, self.player_port.version() + 1),
                        crane_id: ship.assigned_cranes[0], // Representative crane
                        ship_id: ship.id,
                        containers_remaining: remaining,
                    };

                    events.push(event);
                }
            }
        }

        for event in &events {
            self.player_port.apply_event(event);
        }

        self.event_store.append(self.session_id, events).ok();

        // AI port (same logic)
        let mut events = Vec::new();
        for ship in self.ai_port.docked_ships() {
            if !ship.assigned_cranes.is_empty() {
                let crane_count = ship.assigned_cranes.len() as u32;
                let base_amount = crane_count * 10;
                let process_amount = (base_amount as f64 * self.crane_efficiency_modifier) as u32;

                if ship.containers_remaining > 0 {
                    let processed = process_amount.min(ship.containers_remaining);
                    let remaining = ship.containers_remaining - processed;

                    let event = DomainEvent::ContainerProcessed {
                        metadata: EventMetadata::new(self.session_id, self.ai_port.version() + 1),
                        crane_id: ship.assigned_cranes[0],
                        ship_id: ship.id,
                        containers_remaining: remaining,
                    };

                    events.push(event);
                }
            }
        }

        for event in &events {
            self.ai_port.apply_event(event);
        }

        self.event_store.append(self.session_id, events).ok();
    }

    /// AI takes its turn using MCTS
    pub fn ai_take_turn(&mut self) {
        // Get best action from MCTS
        if let Some(action) = self.mcts_engine.search(&self.ai_port) {
            // Apply action to AI port
            match action {
                crate::mcts::MCTSAction::DockShip { ship_id, berth_id } => {
                    use crate::application::handlers::handle_dock_ship_command;

                    if let Ok(events) = handle_dock_ship_command(
                        &self.ai_port,
                        self.session_id,
                        ship_id,
                        berth_id,
                        self.ai_port.player_id,
                    ) {
                        for event in &events {
                            self.ai_port.apply_event(event);
                        }
                        self.event_store.append(self.session_id, events).ok();
                    }
                }
                crate::mcts::MCTSAction::AssignCrane { crane_id, ship_id } => {
                    use crate::application::handlers::handle_assign_crane_command;

                    if let Ok(events) = handle_assign_crane_command(
                        &self.ai_port,
                        self.session_id,
                        crane_id,
                        ship_id,
                        self.ai_port.player_id,
                    ) {
                        for event in &events {
                            self.ai_port.apply_event(event);
                        }
                        self.event_store.append(self.session_id, events).ok();
                    }
                }
                _ => {} // Pass or other actions
            }
        }
    }

    /// Check if game is over (all ships processed)
    pub fn is_game_over(&self) -> bool {
        // Conditions de fin de jeu :
        // 1. Score suffisamment élevé (victoire)
        if self.player_port.score > 1000 {
            return true;
        }

        // 2. Trop de navires en attente (défaite)
        let waiting_ships = self.player_port.waiting_ships().len();
        if waiting_ships > 10 {
            return true;
        }

        // 3. Durée maximum atteinte (30 tours)
        if self.current_turn >= 30 {
            return true;
        }

        false
    }

    /// Get winner (if game is over)
    pub fn get_winner(&self) -> Option<&str> {
        if !self.is_game_over() {
            return None;
        }

        let player_score = self.player_port.calculate_score();
        let ai_score = self.ai_port.calculate_score();

        if player_score > ai_score {
            Some("player")
        } else if ai_score > player_score {
            Some("ai")
        } else {
            Some("tie")
        }
    }

    /// Process random events
    pub fn process_random_events(&mut self) -> Vec<RandomEvent> {
        let mut new_events = Vec::new();

        // Update active events
        self.active_events.retain_mut(|active| {
            let expired = active.tick();
            !expired
        });

        // Reset modifiers
        self.crane_efficiency_modifier = 1.0;

        // Apply active event effects
        for active in &self.active_events {
            match &active.event {
                RandomEvent::Storm { efficiency_penalty, .. } => {
                    self.crane_efficiency_modifier *= 1.0 - efficiency_penalty;
                }
                RandomEvent::GoodWeather { efficiency_bonus, .. } => {
                    self.crane_efficiency_modifier *= 1.0 + efficiency_bonus;
                }
                _ => {}
            }
        }

        // Generate new event
        if let Some(event) = self.event_generator.generate() {
            match &event {
                RandomEvent::RushHour { extra_ships } => {
                    self.spawn_ships(*extra_ships);
                }
                RandomEvent::CustomsInspection { .. } => {
                    // Instant effect - handled in display
                }
                _ => {
                    // Add to active events
                    self.active_events.push(ActiveEvent::new(event.clone()));
                }
            }
            new_events.push(event);
        }

        new_events
    }

    /// Get description of active effects
    pub fn get_active_effects_description(&self) -> Vec<String> {
        self.active_events
            .iter()
            .map(|active| {
                format!(
                    "{} ({}  turns left)",
                    active.event.description(),
                    active.turns_remaining
                )
            })
            .collect()
    }

    /// Free completed ships and their assigned cranes
    pub fn free_completed_ships(&mut self) {
        // Ne récupérer que les navires qui sont complètement déchargés
        let completed_ships: Vec<_> = self.player_port.ships
            .iter()
            .filter(|(_, ship)| {
                ship.is_docked() &&
                ship.containers_remaining == 0  // Uniquement les navires complètement déchargés
            })
            .map(|(id, ship)| (*id, ship.docked_at.unwrap(), ship.assigned_cranes.clone()))
            .collect();

        for (ship_id, berth_id, crane_ids) in completed_ships {
            // Libérer les grues uniquement pour les navires terminés
            for crane_id in crane_ids {
                self.player_port.free_crane(crane_id);
            }
            // Puis libérer le quai
            self.player_port.undock_ship(ship_id, berth_id);
            // Retirer le navire
            self.player_port.ships.remove(&ship_id);
        }
    }

    /// End turn with proper sequence
    pub fn end_turn(&mut self) {
        // 1. Process containers one last time
        self.process_containers();

        // 2. Free completed ships and their assigned cranes
        self.free_completed_ships();

        // 3. Process random events for next turn
        self.process_random_events();

        // 4. Let AI take its turn
        self.ai_take_turn();

        // 5. Start new turn
        self.start_turn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_session_creation() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();

        let session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

        assert_eq!(session.mode, GameMode::VersusAI);
        assert_eq!(session.current_turn, 0);
    }

    #[test]
    fn test_spawn_ships() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();

        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);
        session.spawn_ships(2);

        assert_eq!(session.player_port.ships.len(), 2);
        assert_eq!(session.ai_port.ships.len(), 2);
    }

    #[test]
    fn test_event_export() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();

        let session = GameSession::new(GameMode::VersusAI, player_id, ai_id);
        let json = session.export_replay();

        assert!(json.is_ok());
    }

    #[test]
    fn test_free_completed_ships() {
        let player_id = PlayerId::new();
        let ai_id = PlayerId::new();
        let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

        // 1. Spawn a ship
        session.spawn_ships(1);
        let ship_id = ShipId::new(0);
        let berth_id = BerthId::new(0);

        // 2. Dock it and assign a crane
        session.player_dock_ship(ship_id, berth_id).unwrap();
        session.player_assign_crane(CraneId::new(0), ship_id).unwrap();

        // 3. Empty its containers
        let ship = session.player_port.ships.get_mut(&ship_id).unwrap();
        ship.containers_remaining = 0;

        // 4. Call free_completed_ships
        session.free_completed_ships();

        // 5. Verify:
        // - Ship should be gone
        assert!(!session.player_port.ships.contains_key(&ship_id));
        // - Berth should be free
        let berth = session.player_port.berths.get(&berth_id).unwrap();
        assert!(berth.is_free());
        // - Crane should be unassigned
        let crane = session.player_port.cranes.get(&CraneId::new(0)).unwrap();
        assert!(crane.is_free());
    }
}
