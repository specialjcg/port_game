// Port Game - Interactive MCTS Strategy Game
// Phase 1: Turn-based gameplay with CLI

use port_game::cli::*;
use port_game::domain::value_objects::PlayerId;
use port_game::game::{GameMode, GameSession};

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║         🚢 PORT TERMINAL MANAGER 🚢                         ║");
    println!("║              MCTS Strategy Game                            ║");
    println!("║                                                            ║");
    println!("║  Manage your port efficiently and beat the AI!            ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Initialize game
    let player_id = PlayerId::new();
    let ai_id = PlayerId::new();
    let mut session = GameSession::new(GameMode::VersusAI, player_id, ai_id);

    println!("👤 Your Port ID: {}", player_id);
    println!("🤖 AI Port ID: {}\n", ai_id);

    // Initial ships
    println!("📦 Spawning initial ships...");
    session.spawn_ships(3);
    println!("✅ 3 ships have arrived and are waiting to dock!\n");

    wait_for_enter();

    // Main game loop
    let max_turns = 10;

    for turn in 1..=max_turns {
        clear_screen();
        display_header(turn);

        // Start turn
        session.start_turn();

        // Show current state
        display_port_status(&session.player_port, "📊 YOUR PORT");

        // Player actions loop
        loop {
            display_menu();

            match get_menu_choice() {
                Ok(choice) => match process_player_choice(choice, &session) {
                    Ok(PlayerAction::DockShip { ship_id, berth_id }) => {
                        match session.player_dock_ship(ship_id, berth_id) {
                            Ok(_) => {
                                display_action_result(
                                    true,
                                    &format!("Ship #{} docked at Berth #{}", ship_id.0, berth_id.0),
                                );
                            }
                            Err(e) => {
                                display_action_result(false, &e);
                            }
                        }
                        wait_for_enter();
                    }
                    Ok(PlayerAction::AssignCrane { crane_id, ship_id }) => {
                        match session.player_assign_crane(crane_id, ship_id) {
                            Ok(_) => {
                                display_action_result(
                                    true,
                                    &format!(
                                        "Crane #{} assigned to Ship #{}",
                                        crane_id.0, ship_id.0
                                    ),
                                );
                            }
                            Err(e) => {
                                display_action_result(false, &e);
                            }
                        }
                        wait_for_enter();
                    }
                    Ok(PlayerAction::ViewState) => {
                        clear_screen();
                        display_port_status(&session.player_port, "📊 YOUR PORT");
                        wait_for_enter();
                    }
                    Ok(PlayerAction::ViewComparison) => {
                        clear_screen();
                        display_comparison(&session);
                        wait_for_enter();
                    }
                    Ok(PlayerAction::EndTurn) => {
                        println!("\n⏭️  Ending your turn...");
                        break;
                    }
                    Ok(PlayerAction::Quit) => {
                        if confirm("Are you sure you want to quit?") {
                            println!("\n👋 Thanks for playing!");
                            return;
                        }
                    }
                    Err(e) => {
                        display_action_result(false, &e);
                        wait_for_enter();
                    }
                },
                Err(e) => {
                    display_action_result(false, &e);
                    wait_for_enter();
                }
            }
        }

        // Process random events
        let new_events = session.process_random_events();
        if !new_events.is_empty() {
            println!("\n⚠️  RANDOM EVENT!");
            for event in &new_events {
                println!("   {}", event.description());
            }
            wait_for_enter();
        }

        // Show active effects
        let active_effects = session.get_active_effects_description();
        if !active_effects.is_empty() {
            println!("\n📋 Active Effects:");
            for effect in active_effects {
                println!("   • {}", effect);
            }
        }

        // Process containers
        println!("\n🔄 Processing containers...");
        if session.crane_efficiency_modifier != 1.0 {
            println!(
                "   Crane efficiency: {:.0}%",
                session.crane_efficiency_modifier * 100.0
            );
        }
        session.process_containers();

        // AI turn
        println!("🤖 AI is thinking...");
        session.ai_take_turn();
        println!("✅ AI completed its turn");

        // Show AI port
        display_port_status(&session.ai_port, "🤖 AI PORT");

        // Show comparison
        display_comparison(&session);

        // Check if game over
        if session.is_game_over() {
            println!("\n🎉 All ships have been processed!");
            break;
        }

        // Spawn new ships every 3 turns
        if turn % 3 == 0 && turn < max_turns {
            println!("\n📦 New ships arriving...");
            session.spawn_ships(2);
            println!("✅ 2 new ships have arrived!");
        }

        wait_for_enter();
    }

    // Game end
    clear_screen();
    let winner = session.get_winner();
    display_game_end(&session, winner);

    // Export replay
    if confirm("\n💾 Save game replay to file?") {
        match session.export_replay() {
            Ok(json) => {
                use std::fs;
                let filename = format!("replay_{}.json", session.session_id);
                match fs::write(&filename, json) {
                    Ok(_) => println!("✅ Replay saved to {}", filename),
                    Err(e) => println!("❌ Failed to save replay: {}", e),
                }
            }
            Err(e) => println!("❌ Failed to export replay: {}", e),
        }
    }

    println!("\n👋 Thanks for playing Port Terminal Manager!");
}
