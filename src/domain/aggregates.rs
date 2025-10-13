// Aggregates - DDD pattern for consistency boundaries
// Port is the main aggregate root

use std::collections::HashMap;
use uuid::Uuid;
use super::entities::{Berth, Crane, Ship};
use super::events::{DomainEvent, EventMetadata};
use super::value_objects::{BerthId, CraneId, PlayerId, ShipId};

/// Port aggregate - Manages ships, berths, and cranes
/// This is the consistency boundary and event source
#[derive(Debug, Clone)]
pub struct Port {
    pub player_id: PlayerId,
    pub ships: HashMap<ShipId, Ship>,
    pub berths: HashMap<BerthId, Berth>,
    pub cranes: HashMap<CraneId, Crane>,
    pub current_time: f64,
    pub score: i32,

    // Event sourcing
    version: u64,
    uncommitted_events: Vec<DomainEvent>,
}

impl Port {
    pub fn new(player_id: PlayerId, num_berths: usize, num_cranes: usize) -> Self {
        let mut berths = HashMap::new();
        for i in 0..num_berths {
            berths.insert(BerthId::new(i), Berth::new(BerthId::new(i)));
        }

        let mut cranes = HashMap::new();
        for i in 0..num_cranes {
            cranes.insert(
                CraneId::new(i),
                Crane::new(CraneId::new(i), 2.0), // Default speed
            );
        }

        Self {
            player_id,
            ships: HashMap::new(),
            berths,
            cranes,
            current_time: 0.0,
            score: 0,
            version: 0,
            uncommitted_events: Vec::new(),
        }
    }

    /// Apply an event to update state (Event Sourcing pattern)
    pub fn apply_event(&mut self, event: &DomainEvent) {
        match event {
            DomainEvent::ShipArrived {
                ship_id,
                container_count,
                arrival_time,
                ..
            } => {
                let ship = Ship::new(*ship_id, *container_count, *arrival_time);
                self.ships.insert(*ship_id, ship);
            }

            DomainEvent::ShipDocked {
                ship_id, berth_id, ..
            } => {
                if let Some(ship) = self.ships.get_mut(ship_id) {
                    ship.dock(*berth_id);
                }
                if let Some(berth) = self.berths.get_mut(berth_id) {
                    berth.occupy(*ship_id);
                }
            }

            DomainEvent::ShipUndocked {
                ship_id, berth_id, ..
            } => {
                if let Some(ship) = self.ships.get_mut(ship_id) {
                    ship.undock();
                }
                if let Some(berth) = self.berths.get_mut(berth_id) {
                    berth.free();
                }
                // Ship completed - remove from active ships
                self.ships.remove(ship_id);
            }

            DomainEvent::CraneAssigned {
                crane_id, ship_id, ..
            } => {
                if let Some(crane) = self.cranes.get_mut(crane_id) {
                    crane.assign(*ship_id);
                }
                if let Some(ship) = self.ships.get_mut(ship_id) {
                    ship.assign_crane(*crane_id);
                }
            }

            DomainEvent::CraneUnassigned {
                crane_id, ship_id, ..
            } => {
                if let Some(crane) = self.cranes.get_mut(crane_id) {
                    crane.unassign();
                }
                if let Some(ship) = self.ships.get_mut(ship_id) {
                    ship.unassign_crane(*crane_id);
                }
            }

            DomainEvent::ContainerProcessed {
                ship_id,
                containers_remaining,
                ..
            } => {
                if let Some(ship) = self.ships.get_mut(ship_id) {
                    let containers_processed = ship.containers_remaining - *containers_remaining;
                    ship.containers_remaining = *containers_remaining;
                    // Mise à jour du score : 10 points par conteneur traité
                    self.score += (containers_processed * 10) as i32;
                }
            }

            _ => {} // Other events don't modify port state directly
        }

        self.version += 1;
    }

    /// Get waiting ships (not docked yet)
    pub fn waiting_ships(&self) -> Vec<&Ship> {
        self.ships
            .values()
            .filter(|s| !s.is_docked())
            .collect()
    }

    /// Get docked ships
    pub fn docked_ships(&self) -> Vec<&Ship> {
        self.ships
            .values()
            .filter(|s| s.is_docked())
            .collect()
    }

    /// Get free berths
    pub fn free_berths(&self) -> Vec<&Berth> {
        self.berths.values().filter(|b| b.is_free()).collect()
    }

    /// Get free cranes
    pub fn free_cranes(&self) -> Vec<&Crane> {
        self.cranes.values().filter(|c| c.is_free()).collect()
    }

    /// Calculate current score (simple heuristic)
    pub fn calculate_score(&self) -> i32 {
        let mut score = 0;

        // Positive: containers processed
        for ship in self.ships.values() {
            score += (ship.containers - ship.containers_remaining) as i32 * 10;
        }

        // Negative: waiting time penalty
        for ship in self.waiting_ships() {
            let wait_time = ship.waiting_time(self.current_time);
            score -= (wait_time * 5.0) as i32;
        }

        score
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn uncommitted_events(&self) -> &[DomainEvent] {
        &self.uncommitted_events
    }

    pub fn mark_events_committed(&mut self) {
        self.uncommitted_events.clear();
    }

    pub fn free_crane(&mut self, crane_id: CraneId) {
        if let Some(crane) = self.cranes.get_mut(&crane_id) {
            if let Some(ship_id) = crane.assigned_to {
                let event = DomainEvent::CraneUnassigned {
                    metadata: EventMetadata::new(Uuid::new_v4(), self.version + 1),
                    crane_id,
                    ship_id,
                    unassignment_time: self.current_time,
                };
                self.apply_event(&event);
                self.uncommitted_events.push(event);
            }
        }
    }

    pub fn undock_ship(&mut self, ship_id: ShipId, berth_id: BerthId) {
        if let Some(ship) = self.ships.get(&ship_id) {
            if ship.docked_at == Some(berth_id) {
                let containers_processed = ship.containers - ship.containers_remaining;
                let event = DomainEvent::ShipUndocked {
                    metadata: EventMetadata::new(Uuid::new_v4(), self.version + 1),
                    ship_id,
                    berth_id,
                    completion_time: self.current_time,
                    containers_processed,
                };
                self.apply_event(&event);
                self.uncommitted_events.push(event);
            }
        }
    }

    /// Libère toutes les grues du port
    pub fn free_all_cranes(&mut self) {
        for crane in self.cranes.values_mut() {
            crane.unassign();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::events::EventMetadata;
    use uuid::Uuid;

    #[test]
    fn test_port_creation() {
        let player_id = PlayerId::new();
        let port = Port::new(player_id, 2, 2);

        assert_eq!(port.berths.len(), 2);
        assert_eq!(port.cranes.len(), 2);
        assert_eq!(port.ships.len(), 0);
        assert_eq!(port.version, 0);
    }

    #[test]
    fn test_ship_arrival_event() {
        let player_id = PlayerId::new();
        let mut port = Port::new(player_id, 2, 2);

        let event = DomainEvent::ShipArrived {
            metadata: EventMetadata::new(Uuid::new_v4(), 1),
            ship_id: ShipId::new(1),
            container_count: 50,
            arrival_time: 0.0,
        };

        port.apply_event(&event);

        assert_eq!(port.ships.len(), 1);
        assert_eq!(port.version, 1);
    }

    #[test]
    fn test_ship_docking_event() {
        let player_id = PlayerId::new();
        let mut port = Port::new(player_id, 2, 2);

        // First ship arrives
        let arrival_event = DomainEvent::ShipArrived {
            metadata: EventMetadata::new(Uuid::new_v4(), 1),
            ship_id: ShipId::new(1),
            container_count: 50,
            arrival_time: 0.0,
        };
        port.apply_event(&arrival_event);

        // Then it docks
        let dock_event = DomainEvent::ShipDocked {
            metadata: EventMetadata::new(Uuid::new_v4(), 2),
            ship_id: ShipId::new(1),
            berth_id: BerthId::new(0),
            player: player_id,
            docking_time: 1.0,
        };
        port.apply_event(&dock_event);

        assert_eq!(port.waiting_ships().len(), 0);
        assert_eq!(port.docked_ships().len(), 1);
        assert_eq!(port.free_berths().len(), 1);
    }

    #[test]
    fn test_crane_assignment() {
        let player_id = PlayerId::new();
        let mut port = Port::new(player_id, 2, 2);

        let arrival_event = DomainEvent::ShipArrived {
            metadata: EventMetadata::new(Uuid::new_v4(), 1),
            ship_id: ShipId::new(1),
            container_count: 50,
            arrival_time: 0.0,
        };
        port.apply_event(&arrival_event);

        let dock_event = DomainEvent::ShipDocked {
            metadata: EventMetadata::new(Uuid::new_v4(), 2),
            ship_id: ShipId::new(1),
            berth_id: BerthId::new(0),
            player: player_id,
            docking_time: 1.0,
        };
        port.apply_event(&dock_event);

        let crane_event = DomainEvent::CraneAssigned {
            metadata: EventMetadata::new(Uuid::new_v4(), 3),
            crane_id: CraneId::new(0),
            ship_id: ShipId::new(1),
            player: player_id,
            assignment_time: 2.0,
        };
        port.apply_event(&crane_event);

        assert_eq!(port.free_cranes().len(), 1);

        let ship = port.ships.get(&ShipId::new(1)).unwrap();
        assert_eq!(ship.assigned_cranes.len(), 1);
    }

    #[test]
    fn test_free_crane() {
        let player_id = PlayerId::new();
        let mut port = Port::new(player_id, 2, 2);

        let arrival_event = DomainEvent::ShipArrived {
            metadata: EventMetadata::new(Uuid::new_v4(), 1),
            ship_id: ShipId::new(1),
            container_count: 50,
            arrival_time: 0.0,
        };
        port.apply_event(&arrival_event);

        let dock_event = DomainEvent::ShipDocked {
            metadata: EventMetadata::new(Uuid::new_v4(), 2),
            ship_id: ShipId::new(1),
            berth_id: BerthId::new(0),
            player: player_id,
            docking_time: 1.0,
        };
        port.apply_event(&dock_event);

        let crane_event = DomainEvent::CraneAssigned {
            metadata: EventMetadata::new(Uuid::new_v4(), 3),
            crane_id: CraneId::new(0),
            ship_id: ShipId::new(1),
            player: player_id,
            assignment_time: 2.0,
        };
        port.apply_event(&crane_event);

        assert_eq!(port.free_cranes().len(), 1);

        port.free_crane(CraneId::new(0));

        assert_eq!(port.free_cranes().len(), 2);
    }

    #[test]
    fn test_undock_ship() {
        let player_id = PlayerId::new();
        let mut port = Port::new(player_id, 2, 2);

        let arrival_event = DomainEvent::ShipArrived {
            metadata: EventMetadata::new(Uuid::new_v4(), 1),
            ship_id: ShipId::new(1),
            container_count: 50,
            arrival_time: 0.0,
        };
        port.apply_event(&arrival_event);

        let dock_event = DomainEvent::ShipDocked {
            metadata: EventMetadata::new(Uuid::new_v4(), 2),
            ship_id: ShipId::new(1),
            berth_id: BerthId::new(0),
            player: player_id,
            docking_time: 1.0,
        };
        port.apply_event(&dock_event);

        port.undock_ship(ShipId::new(1), BerthId::new(0));

        assert_eq!(port.ships.len(), 0);
        assert_eq!(port.free_berths().len(), 2);
    }
}
