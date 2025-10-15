// Event Store - Persistence for Event Sourcing
// In-memory implementation for MVP, can be replaced with DB later

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::domain::events::DomainEvent;

/// Event store trait for dependency inversion
pub trait EventStore: Send + Sync {
    fn append(&mut self, aggregate_id: Uuid, events: Vec<DomainEvent>) -> Result<(), String>;
    fn load(&self, aggregate_id: Uuid) -> Result<Vec<DomainEvent>, String>;
    fn all_events(&self) -> Vec<DomainEvent>;
}

/// In-memory event store for MVP
#[derive(Debug, Clone)]
pub struct InMemoryEventStore {
    events: Arc<RwLock<HashMap<Uuid, Vec<DomainEvent>>>>,
}

impl InMemoryEventStore {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Export events to JSON (for replay/debugging)
    pub fn export_to_json(&self, aggregate_id: Uuid) -> Result<String, String> {
        let events = self.load(aggregate_id)?;
        serde_json::to_string_pretty(&events).map_err(|e| e.to_string())
    }

    /// Import events from JSON
    pub fn import_from_json(&mut self, aggregate_id: Uuid, json: &str) -> Result<(), String> {
        let events: Vec<DomainEvent> = serde_json::from_str(json).map_err(|e| e.to_string())?;
        self.append(aggregate_id, events)
    }
}

impl Default for InMemoryEventStore {
    fn default() -> Self {
        Self::new()
    }
}

impl EventStore for InMemoryEventStore {
    fn append(&mut self, aggregate_id: Uuid, events: Vec<DomainEvent>) -> Result<(), String> {
        let mut store = self.events.write().map_err(|e| e.to_string())?;

        store
            .entry(aggregate_id)
            .or_insert_with(Vec::new)
            .extend(events);

        Ok(())
    }

    fn load(&self, aggregate_id: Uuid) -> Result<Vec<DomainEvent>, String> {
        let store = self.events.read().map_err(|e| e.to_string())?;

        Ok(store.get(&aggregate_id).cloned().unwrap_or_default())
    }

    fn all_events(&self) -> Vec<DomainEvent> {
        let store = self.events.read().unwrap();
        store.values().flat_map(|events| events.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::events::EventMetadata;
    use crate::domain::value_objects::ShipId;

    #[test]
    fn test_append_and_load() {
        let mut store = InMemoryEventStore::new();
        let aggregate_id = Uuid::new_v4();

        let event = DomainEvent::ShipArrived {
            metadata: EventMetadata::new(aggregate_id, 1),
            ship_id: ShipId::new(1),
            container_count: 50,
            arrival_time: 0.0,
        };

        store.append(aggregate_id, vec![event.clone()]).unwrap();

        let loaded = store.load(aggregate_id).unwrap();

        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].event_type(), event.event_type());
    }

    #[test]
    fn test_multiple_aggregates() {
        let mut store = InMemoryEventStore::new();
        let agg1 = Uuid::new_v4();
        let agg2 = Uuid::new_v4();

        let event1 = DomainEvent::ShipArrived {
            metadata: EventMetadata::new(agg1, 1),
            ship_id: ShipId::new(1),
            container_count: 50,
            arrival_time: 0.0,
        };

        let event2 = DomainEvent::ShipArrived {
            metadata: EventMetadata::new(agg2, 1),
            ship_id: ShipId::new(2),
            container_count: 30,
            arrival_time: 0.0,
        };

        store.append(agg1, vec![event1]).unwrap();
        store.append(agg2, vec![event2]).unwrap();

        assert_eq!(store.load(agg1).unwrap().len(), 1);
        assert_eq!(store.load(agg2).unwrap().len(), 1);
        assert_eq!(store.all_events().len(), 2);
    }

    #[test]
    fn test_json_export_import() {
        let mut store = InMemoryEventStore::new();
        let aggregate_id = Uuid::new_v4();

        let event = DomainEvent::ShipArrived {
            metadata: EventMetadata::new(aggregate_id, 1),
            ship_id: ShipId::new(1),
            container_count: 50,
            arrival_time: 0.0,
        };

        store.append(aggregate_id, vec![event]).unwrap();

        let json = store.export_to_json(aggregate_id).unwrap();

        let mut new_store = InMemoryEventStore::new();
        new_store.import_from_json(aggregate_id, &json).unwrap();

        let loaded = new_store.load(aggregate_id).unwrap();
        assert_eq!(loaded.len(), 1);
    }
}
