// Display utilities for CLI

use crate::domain::aggregates::Port;
use crate::game::GameSession;

/// Display port status with nice formatting
pub fn display_port_status(port: &Port, title: &str) {
    println!("\n┌────────────────────────────────────────────────────────────┐");
    println!("│ {}                                      ", title);
    println!("├────────────────────────────────────────────────────────────┤");
    println!("│ 🏗️  Berths: {} total, {} free                              ",
        port.berths.len(),
        port.free_berths().len()
    );
    println!("│ 🏗️  Cranes: {} total, {} free                              ",
        port.cranes.len(),
        port.free_cranes().len()
    );
    println!("│ 🚢 Ships: {} waiting, {} docked                           ",
        port.waiting_ships().len(),
        port.docked_ships().len()
    );
    println!("│ 🎯 Score: {}                                               ", port.calculate_score());
    println!("└────────────────────────────────────────────────────────────┘");

    // Show waiting ships
    if !port.waiting_ships().is_empty() {
        println!("\n📦 WAITING SHIPS:");
        for ship in port.waiting_ships() {
            println!(
                "  • Ship #{}: {} containers (waiting {:.1}s)",
                ship.id.0,
                ship.containers,
                ship.waiting_time(port.current_time)
            );
        }
    }

    // Show docked ships
    if !port.docked_ships().is_empty() {
        println!("\n⚓ DOCKED SHIPS:");
        for ship in port.docked_ships() {
            let berth_id = ship.docked_at.unwrap().0;
            let assigned_cranes: Vec<_> = ship.assigned_cranes.iter().map(|c| c.0).collect();
            println!(
                "  • Ship #{} at Berth #{}: {}/{} containers | Cranes: {:?}",
                ship.id.0,
                berth_id,
                ship.containers_remaining,
                ship.containers,
                if assigned_cranes.is_empty() {
                    vec![]
                } else {
                    assigned_cranes
                }
            );
        }
    }
}

/// Display comparison between player and AI
pub fn display_comparison(session: &GameSession) {
    let player_score = session.player_port.calculate_score();
    let ai_score = session.ai_port.calculate_score();

    let player_ships_done = session.player_port.ships.values()
        .filter(|s| s.is_completed())
        .count();
    let ai_ships_done = session.ai_port.ships.values()
        .filter(|s| s.is_completed())
        .count();

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║               PLAYER vs AI COMPARISON                      ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║                    PLAYER    │    AI                       ║");
    println!("╟────────────────────────────────────────────────────────────╢");
    println!("║ Score:              {:6}   │  {:6}                     ║", player_score, ai_score);
    println!("║ Ships completed:    {:6}   │  {:6}                     ║", player_ships_done, ai_ships_done);
    println!("║ Ships waiting:      {:6}   │  {:6}                     ║",
        session.player_port.waiting_ships().len(),
        session.ai_port.waiting_ships().len()
    );
    println!("║ Ships docked:       {:6}   │  {:6}                     ║",
        session.player_port.docked_ships().len(),
        session.ai_port.docked_ships().len()
    );
    println!("╚════════════════════════════════════════════════════════════╝");

    if player_score > ai_score {
        println!("🏆 You are WINNING! (+{})", player_score - ai_score);
    } else if ai_score > player_score {
        println!("⚠️  AI is ahead! (-{})", ai_score - player_score);
    } else {
        println!("🤝 It's a TIE!");
    }
}

/// Display game header
pub fn display_header(turn: u32) {
    println!("\n");
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║         🚢 PORT TERMINAL MANAGER 🚢                         ║");
    println!("║              MCTS Strategy Game                            ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║ Turn: {}                                                    ║", turn);
    println!("╚════════════════════════════════════════════════════════════╝");
}

/// Display action result
pub fn display_action_result(success: bool, message: &str) {
    if success {
        println!("✅ {}", message);
    } else {
        println!("❌ {}", message);
    }
}

/// Display game end
pub fn display_game_end(session: &GameSession, winner: Option<&str>) {
    println!("\n");
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                    GAME OVER                               ║");
    println!("╠════════════════════════════════════════════════════════════╣");

    let player_score = session.player_port.calculate_score();
    let ai_score = session.ai_port.calculate_score();

    println!("║ Final Score:                                               ║");
    println!("║   Player: {:6}                                            ║", player_score);
    println!("║   AI:     {:6}                                            ║", ai_score);
    println!("╠════════════════════════════════════════════════════════════╣");

    match winner {
        Some("player") => {
            println!("║            🏆 YOU WIN! 🏆                                  ║");
        }
        Some("ai") => {
            println!("║            🤖 AI WINS! 🤖                                  ║");
        }
        _ => {
            println!("║            🤝 IT'S A TIE! 🤝                               ║");
        }
    }

    println!("╚════════════════════════════════════════════════════════════╝");
}
