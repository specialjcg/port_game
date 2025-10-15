// Random game events system
// Adds unpredictability and challenge to the game

use crate::domain::value_objects::CraneId;
use crate::utils::random;

/// Random events that can occur during gameplay
#[derive(Debug, Clone, PartialEq)]
pub enum RandomEvent {
    /// Storm reduces crane efficiency
    Storm {
        duration_turns: u32,
        efficiency_penalty: f64, // 0.0 to 1.0
    },

    /// Crane breakdown - crane unavailable
    CraneBreakdown {
        crane_id: CraneId,
        duration_turns: u32,
    },

    /// Customs inspection - random ship delayed
    CustomsInspection { delay_turns: u32 },

    /// Rush hour - multiple ships arrive
    RushHour { extra_ships: usize },

    /// Good weather - bonus efficiency
    GoodWeather {
        duration_turns: u32,
        efficiency_bonus: f64,
    },
}

impl RandomEvent {
    pub fn description(&self) -> String {
        match self {
            RandomEvent::Storm {
                duration_turns,
                efficiency_penalty,
            } => {
                format!(
                    "🌊 STORM! Crane efficiency reduced by {:.0}% for {} turns",
                    efficiency_penalty * 100.0,
                    duration_turns
                )
            }
            RandomEvent::CraneBreakdown {
                crane_id,
                duration_turns,
            } => {
                format!(
                    "🔧 BREAKDOWN! Crane #{} is out of service for {} turns",
                    crane_id.0, duration_turns
                )
            }
            RandomEvent::CustomsInspection { delay_turns } => {
                format!(
                    "👮 CUSTOMS INSPECTION! Next ship delayed by {} turns",
                    delay_turns
                )
            }
            RandomEvent::RushHour { extra_ships } => {
                format!("⚡ RUSH HOUR! {} additional ships arriving!", extra_ships)
            }
            RandomEvent::GoodWeather {
                duration_turns,
                efficiency_bonus,
            } => {
                format!(
                    "☀️ GOOD WEATHER! Crane efficiency increased by {:.0}% for {} turns",
                    efficiency_bonus * 100.0,
                    duration_turns
                )
            }
        }
    }
}

/// Event generator with configurable probability
pub struct EventGenerator {
    probability: f64, // 0.0 to 1.0
}

impl EventGenerator {
    pub fn new(probability: f64) -> Self {
        Self {
            probability: probability.clamp(0.0, 1.0),
        }
    }

    /// Generate a random event (or None)
    pub fn generate(&self) -> Option<RandomEvent> {
        // Check if event should occur
        if !random::hit(self.probability) {
            return None;
        }

        // Choose event type
        let event_type = random::range_usize(0, 5);

        match event_type {
            0 => Some(RandomEvent::Storm {
                duration_turns: random::range_u32_inclusive(1, 3),
                efficiency_penalty: random::range_f64_inclusive(0.3, 0.6),
            }),
            1 => Some(RandomEvent::CraneBreakdown {
                crane_id: CraneId::new(random::range_usize(0, 2)), // Assume 2 cranes
                duration_turns: random::range_u32_inclusive(1, 2),
            }),
            2 => Some(RandomEvent::CustomsInspection {
                delay_turns: random::range_u32_inclusive(1, 2),
            }),
            3 => Some(RandomEvent::RushHour {
                extra_ships: random::range_usize_inclusive(1, 3),
            }),
            4 => Some(RandomEvent::GoodWeather {
                duration_turns: random::range_u32_inclusive(1, 2),
                efficiency_bonus: random::range_f64_inclusive(0.2, 0.4),
            }),
            _ => None,
        }
    }
}

impl Default for EventGenerator {
    fn default() -> Self {
        Self::new(0.3) // 30% chance per turn
    }
}

/// Active event tracker
#[derive(Debug, Clone)]
pub struct ActiveEvent {
    pub event: RandomEvent,
    pub turns_remaining: u32,
}

impl ActiveEvent {
    pub fn new(event: RandomEvent) -> Self {
        let turns_remaining = match &event {
            RandomEvent::Storm { duration_turns, .. } => *duration_turns,
            RandomEvent::CraneBreakdown { duration_turns, .. } => *duration_turns,
            RandomEvent::GoodWeather { duration_turns, .. } => *duration_turns,
            _ => 0, // Instant events
        };

        Self {
            event,
            turns_remaining,
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.turns_remaining > 0 {
            self.turns_remaining -= 1;
        }
        self.turns_remaining == 0
    }

    pub fn is_expired(&self) -> bool {
        self.turns_remaining == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_description() {
        let event = RandomEvent::Storm {
            duration_turns: 2,
            efficiency_penalty: 0.5,
        };

        let desc = event.description();
        assert!(desc.contains("STORM"));
        assert!(desc.contains("50%"));
    }

    #[test]
    fn test_event_generator() {
        let generator = EventGenerator::new(1.0); // 100% chance
        let event = generator.generate();
        assert!(event.is_some());
    }

    #[test]
    fn test_active_event_tick() {
        let event = RandomEvent::Storm {
            duration_turns: 2,
            efficiency_penalty: 0.5,
        };

        let mut active = ActiveEvent::new(event);
        assert_eq!(active.turns_remaining, 2);

        assert!(!active.tick()); // Turn 1
        assert_eq!(active.turns_remaining, 1);

        assert!(active.tick()); // Turn 2 - expired
        assert_eq!(active.turns_remaining, 0);
    }
}
