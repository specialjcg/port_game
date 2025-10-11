// Port Game - MCTS Strategy Game
// CLI interface for MVP

use port_game::domain::value_objects::PlayerId;
use port_game::game::{GameMode, GameSession};

fn main() {
    println!("🚢 PORT TERMINAL MANAGER - MCTS Strategy Game");
    println!("==============================================\n");

    // Create game session
    let player_id = PlayerId::new();
    let ai_id = PlayerId::new();

    let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

    println!("🎮 Game Mode: Player vs AI");
    println!("👤 Player ID: {}", player_id);
    println!("🤖 AI ID: {}\n", ai_id);

    // Start game
    println!("🎬 Starting game...");
    session.start_turn();

    println!("📦 Spawning ships...");
    session.spawn_ships(3);

    // Display initial state
    println!("\n📊 PLAYER PORT STATUS:");
    display_port_status(session.get_player_port());

    println!("\n📊 AI PORT STATUS:");
    display_port_status(session.get_ai_port());

    // Export replay
    match session.export_replay() {
        Ok(json) => {
            println!("\n💾 Game replay exported successfully!");
            println!("📝 Event count: {} events", json.lines().count());
        }
        Err(e) => eprintln!("❌ Failed to export replay: {}", e),
    }

    println!("\n✅ Game initialized successfully!");
    println!("🚀 Next step: Implement interactive CLI for player actions");
}

fn display_port_status(port: &port_game::domain::aggregates::Port) {
    println!("  Berths: {}", port.berths.len());
    println!("  Cranes: {}", port.cranes.len());
    println!("  Ships waiting: {}", port.waiting_ships().len());
    println!("  Ships docked: {}", port.docked_ships().len());
    println!("  Score: {}", port.calculate_score());

    if !port.ships.is_empty() {
        println!("\n  Ships:");
        for ship in port.ships.values() {
            println!(
                "    - Ship #{}: {} containers ({})",
                ship.id.0,
                ship.containers,
                if ship.is_docked() { "docked" } else { "waiting" }
            );
        }
    }
}
