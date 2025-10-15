// Integration tests for random events system

use port_game::domain::value_objects::{CraneId, PlayerId};
use port_game::game::{ActiveEvent, EventGenerator, GameMode, GameSession, RandomEvent};

#[test]
fn test_event_generator_creates_events() {
    let generator = EventGenerator::new(1.0); // 100% probability
    let event = generator.generate();
    assert!(event.is_some());
}

#[test]
fn test_event_generator_respects_probability() {
    let generator = EventGenerator::new(0.0); // 0% probability

    let mut events_generated = 0;
    for _ in 0..100 {
        if generator.generate().is_some() {
            events_generated += 1;
        }
    }

    // With 0% probability, should generate very few or no events
    assert!(events_generated < 5);
}

#[test]
fn test_storm_event_reduces_efficiency() {
    let player_id = PlayerId::new();
    let ai_id = PlayerId::new();
    let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

    // Manually add a storm event
    let storm = RandomEvent::Storm {
        duration_turns: 2,
        efficiency_penalty: 0.5,
    };
    session.active_events.push(ActiveEvent::new(storm));

    // Process events to apply effects
    session.process_random_events();

    // Efficiency should be reduced
    assert_eq!(session.crane_efficiency_modifier, 0.5);
}

#[test]
fn test_good_weather_increases_efficiency() {
    let player_id = PlayerId::new();
    let ai_id = PlayerId::new();
    let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

    let good_weather = RandomEvent::GoodWeather {
        duration_turns: 2,
        efficiency_bonus: 0.3,
    };
    session.active_events.push(ActiveEvent::new(good_weather));

    session.process_random_events();

    // Efficiency should be increased
    assert_eq!(session.crane_efficiency_modifier, 1.3);
}

#[test]
fn test_active_event_countdown() {
    let event = RandomEvent::Storm {
        duration_turns: 3,
        efficiency_penalty: 0.5,
    };

    let mut active = ActiveEvent::new(event);
    assert_eq!(active.turns_remaining, 3);

    assert!(!active.tick()); // Turn 1
    assert_eq!(active.turns_remaining, 2);

    assert!(!active.tick()); // Turn 2
    assert_eq!(active.turns_remaining, 1);

    assert!(active.tick()); // Turn 3 - expired
    assert_eq!(active.turns_remaining, 0);
    assert!(active.is_expired());
}

#[test]
fn test_rush_hour_spawns_ships() {
    let player_id = PlayerId::new();
    let ai_id = PlayerId::new();
    let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

    let initial_ships = session.player_port.ships.len();

    // RushHour is a one-time event that triggers spawn_ships directly
    // Simulate what process_random_events does when it gets a RushHour event
    let extra_ships = 3;
    session.spawn_ships(extra_ships);

    // Should have spawned extra ships
    assert_eq!(session.player_port.ships.len(), initial_ships + extra_ships);
}

#[test]
fn test_multiple_active_effects_stack() {
    let player_id = PlayerId::new();
    let ai_id = PlayerId::new();
    let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

    // Add storm (-50%)
    let storm = RandomEvent::Storm {
        duration_turns: 2,
        efficiency_penalty: 0.5,
    };
    session.active_events.push(ActiveEvent::new(storm));

    // Add good weather (+30%)
    let good_weather = RandomEvent::GoodWeather {
        duration_turns: 2,
        efficiency_bonus: 0.3,
    };
    session.active_events.push(ActiveEvent::new(good_weather));

    session.process_random_events();

    // Effects should stack: 0.5 * 1.3 = 0.65
    assert!((session.crane_efficiency_modifier - 0.65).abs() < 0.01);
}

#[test]
fn test_event_descriptions() {
    let storm = RandomEvent::Storm {
        duration_turns: 2,
        efficiency_penalty: 0.5,
    };
    assert!(storm.description().contains("STORM"));
    assert!(storm.description().contains("50%"));

    let breakdown = RandomEvent::CraneBreakdown {
        crane_id: CraneId::new(0),
        duration_turns: 1,
    };
    assert!(breakdown.description().contains("BREAKDOWN"));
    assert!(breakdown.description().contains("Crane #0"));

    let customs = RandomEvent::CustomsInspection { delay_turns: 1 };
    assert!(customs.description().contains("CUSTOMS"));

    let rush = RandomEvent::RushHour { extra_ships: 2 };
    assert!(rush.description().contains("RUSH HOUR"));
    assert!(rush.description().contains("2"));

    let weather = RandomEvent::GoodWeather {
        duration_turns: 1,
        efficiency_bonus: 0.2,
    };
    assert!(weather.description().contains("GOOD WEATHER"));
}

#[test]
fn test_events_expire_after_duration() {
    let player_id = PlayerId::new();
  	let ai_id = PlayerId::new();
    let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

    // Désactiver la génération aléatoire pour un test déterministe
    session.event_generator = EventGenerator::new(0.0);

    let storm = RandomEvent::Storm {
        duration_turns: 2, // Need 2 turns to see the expiration
        efficiency_penalty: 0.5,
    };
    session.active_events.push(ActiveEvent::new(storm));

    // First turn - storm active (2 turns remaining -> 1)
    session.process_random_events();
    assert_eq!(session.crane_efficiency_modifier, 0.5);
    assert_eq!(session.active_events.len(), 1);
    assert_eq!(session.active_events[0].turns_remaining, 1);

    // Second turn - storm still active (1 turn remaining -> 0, then removed)
    session.process_random_events();
    assert_eq!(session.crane_efficiency_modifier, 1.0); // Back to normal
    assert_eq!(session.active_events.len(), 0); // Storm expired and removed
}

#[test]
fn test_container_processing_with_modified_efficiency() {
    let player_id = PlayerId::new();
    let ai_id = PlayerId::new();
    let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

    // Créer et docker un navire
    session.spawn_ships(1);
    let ship_id = *session.player_port.ships.iter().next().unwrap().0;
    let berth_id = *session.player_port.berths.iter().next().unwrap().0;
    let crane_id = *session.player_port.cranes.iter().next().unwrap().0;

    // Docker le navire et assigner une grue
    session.player_dock_ship(ship_id, berth_id).unwrap();
    session.player_assign_crane(crane_id, ship_id).unwrap();

    // Désactiver les événements aléatoires pour garder le test déterministe
    session.event_generator = EventGenerator::new(0.0);

    // Ajouter un événement de tempête qui réduit l'efficacité de 50%
    let storm = RandomEvent::Storm {
        duration_turns: 2,
        efficiency_penalty: 0.5,
    };
    session.active_events.push(ActiveEvent::new(storm));

    // S'assurer que le modificateur est appliqué
    session.process_random_events();

    let initial_containers = session
        .player_port
        .ships
        .get(&ship_id)
        .unwrap()
        .containers_remaining;

    // Traiter les conteneurs avec l'efficacité réduite
    session.process_containers();

    let final_containers = session
        .player_port
        .ships
        .get(&ship_id)
        .unwrap()
        .containers_remaining;
    let processed = initial_containers - final_containers;

    // Une grue traite normalement 10 conteneurs, avec 50% d'efficacité, elle devrait en traiter 5
    assert_eq!(
        processed, 5,
        "Devrait traiter 5 conteneurs avec l'efficacité réduite de 50%"
    );
}
