// Domain Entities - Business objects with identity
// Following DDD principles

use serde::{Deserialize, Serialize};

use super::value_objects::{BerthId, CraneId, ShipId};

/// Ship entity - Represents a cargo ship waiting to dock
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship {
    pub id: ShipId,
    pub containers: u32,
    pub containers_remaining: u32,
    pub arrival_time: f64,
    pub docked_at: Option<BerthId>,
    pub assigned_cranes: Vec<CraneId>,
}

impl Ship {
    pub fn new(id: ShipId, containers: u32, arrival_time: f64) -> Self {
        Self {
            id,
            containers,
            containers_remaining: containers,
            arrival_time,
            docked_at: None,
            assigned_cranes: Vec::new(),
        }
    }

    pub fn is_docked(&self) -> bool {
        self.docked_at.is_some()
    }

    pub fn is_completed(&self) -> bool {
        self.containers_remaining == 0
    }

    pub fn dock(&mut self, berth_id: BerthId) {
        self.docked_at = Some(berth_id);
    }

    pub fn undock(&mut self) {
        self.docked_at = None;
        self.assigned_cranes.clear();
    }

    pub fn assign_crane(&mut self, crane_id: CraneId) {
        if !self.assigned_cranes.contains(&crane_id) {
            self.assigned_cranes.push(crane_id);
        }
    }

    pub fn unassign_crane(&mut self, crane_id: CraneId) {
        self.assigned_cranes.retain(|&c| c != crane_id);
    }

    pub fn process_containers(&mut self, count: u32) {
        self.containers_remaining = self.containers_remaining.saturating_sub(count);
    }

    pub fn waiting_time(&self, current_time: f64) -> f64 {
        current_time - self.arrival_time
    }
}

/// Berth entity - Docking position for ships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Berth {
    pub id: BerthId,
    pub occupied_by: Option<ShipId>,
}

impl Berth {
    pub fn new(id: BerthId) -> Self {
        Self {
            id,
            occupied_by: None,
        }
    }

    pub fn is_free(&self) -> bool {
        self.occupied_by.is_none()
    }

    pub fn occupy(&mut self, ship_id: ShipId) {
        self.occupied_by = Some(ship_id);
    }

    pub fn free(&mut self) {
        self.occupied_by = None;
    }
}

/// Crane entity - Equipment for unloading containers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crane {
    pub id: CraneId,
    pub assigned_to: Option<ShipId>,
    pub processing_speed: f64, // containers per time unit
}

impl Crane {
    pub fn new(id: CraneId, processing_speed: f64) -> Self {
        Self {
            id,
            assigned_to: None,
            processing_speed,
        }
    }

    pub fn is_free(&self) -> bool {
        self.assigned_to.is_none()
    }

    pub fn assign(&mut self, ship_id: ShipId) {
        self.assigned_to = Some(ship_id);
    }

    pub fn unassign(&mut self) {
        self.assigned_to = None;
    }

    pub fn containers_per_turn(&self) -> u32 {
        (self.processing_speed * 10.0) as u32 // Simple formula
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship_creation() {
        let ship = Ship::new(ShipId::new(1), 50, 0.0);

        assert_eq!(ship.id, ShipId::new(1));
        assert_eq!(ship.containers, 50);
        assert_eq!(ship.containers_remaining, 50);
        assert!(!ship.is_docked());
        assert!(!ship.is_completed());
    }

    #[test]
    fn test_ship_docking() {
        let mut ship = Ship::new(ShipId::new(1), 50, 0.0);
        let berth = BerthId::new(1);

        ship.dock(berth);

        assert!(ship.is_docked());
        assert_eq!(ship.docked_at, Some(berth));
    }

    #[test]
    fn test_crane_assignment() {
        let mut ship = Ship::new(ShipId::new(1), 50, 0.0);
        let crane = CraneId::new(1);

        ship.assign_crane(crane);

        assert_eq!(ship.assigned_cranes.len(), 1);
        assert!(ship.assigned_cranes.contains(&crane));
    }

    #[test]
    fn test_container_processing() {
        let mut ship = Ship::new(ShipId::new(1), 50, 0.0);

        ship.process_containers(20);

        assert_eq!(ship.containers_remaining, 30);
        assert!(!ship.is_completed());

        ship.process_containers(30);

        assert_eq!(ship.containers_remaining, 0);
        assert!(ship.is_completed());
    }

    #[test]
    fn test_berth_occupation() {
        let mut berth = Berth::new(BerthId::new(1));
        let ship_id = ShipId::new(1);

        assert!(berth.is_free());

        berth.occupy(ship_id);

        assert!(!berth.is_free());
        assert_eq!(berth.occupied_by, Some(ship_id));

        berth.free();

        assert!(berth.is_free());
    }

    #[test]
    fn test_crane_availability() {
        let mut crane = Crane::new(CraneId::new(1), 2.0);

        assert!(crane.is_free());

        crane.assign(ShipId::new(1));

        assert!(!crane.is_free());
        assert_eq!(crane.assigned_to, Some(ShipId::new(1)));

        crane.unassign();

        assert!(crane.is_free());
    }
}
