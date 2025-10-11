// CLI module - Interactive command-line interface

pub mod display;
pub mod input;

use std::io::{self, Write};

use crate::domain::aggregates::Port;
use crate::domain::value_objects::{BerthId, CraneId, ShipId};
use crate::game::GameSession;

pub use display::*;
pub use input::*;

/// Player action menu
#[derive(Debug, Clone, PartialEq)]
pub enum PlayerAction {
    DockShip { ship_id: ShipId, berth_id: BerthId },
    AssignCrane { crane_id: CraneId, ship_id: ShipId },
    ViewState,
    ViewComparison,
    EndTurn,
    Quit,
}

/// Display main menu and get player choice
pub fn display_menu() {
    println!("\n┌────────────────────────────────────┐");
    println!("│       AVAILABLE ACTIONS            │");
    println!("├────────────────────────────────────┤");
    println!("│ 1. Dock a ship                     │");
    println!("│ 2. Assign crane to ship            │");
    println!("│ 3. View port state                 │");
    println!("│ 4. View player vs AI comparison    │");
    println!("│ 5. End turn                        │");
    println!("│ 6. Quit game                       │");
    println!("└────────────────────────────────────┘");
    print!("Choose action (1-6): ");
    io::stdout().flush().unwrap();
}

/// Get player input for menu choice
pub fn get_menu_choice() -> Result<u32, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    input
        .trim()
        .parse::<u32>()
        .map_err(|_| "Please enter a valid number".to_string())
}

/// Handle dock ship action
pub fn handle_dock_ship_input(port: &Port) -> Result<PlayerAction, String> {
    println!("\n=== DOCK SHIP ===");

    // Show available ships
    let waiting_ships = port.waiting_ships();
    if waiting_ships.is_empty() {
        return Err("No ships waiting to dock!".to_string());
    }

    println!("\nWaiting ships:");
    for (i, ship) in waiting_ships.iter().enumerate() {
        println!(
            "  {}. Ship #{} - {} containers",
            i + 1,
            ship.id.0,
            ship.containers
        );
    }

    print!("\nSelect ship number: ");
    io::stdout().flush().unwrap();
    let ship_idx = get_user_index()? - 1;

    if ship_idx >= waiting_ships.len() {
        return Err("Invalid ship number".to_string());
    }

    let ship_id = waiting_ships[ship_idx].id;

    // Show available berths
    let free_berths = port.free_berths();
    if free_berths.is_empty() {
        return Err("No berths available!".to_string());
    }

    println!("\nAvailable berths:");
    for (i, berth) in free_berths.iter().enumerate() {
        println!("  {}. Berth #{}", i + 1, berth.id.0);
    }

    print!("\nSelect berth number: ");
    io::stdout().flush().unwrap();
    let berth_idx = get_user_index()? - 1;

    if berth_idx >= free_berths.len() {
        return Err("Invalid berth number".to_string());
    }

    let berth_id = free_berths[berth_idx].id;

    Ok(PlayerAction::DockShip { ship_id, berth_id })
}

/// Handle assign crane action
pub fn handle_assign_crane_input(port: &Port) -> Result<PlayerAction, String> {
    println!("\n=== ASSIGN CRANE ===");

    // Show docked ships
    let docked_ships = port.docked_ships();
    if docked_ships.is_empty() {
        return Err("No ships docked yet!".to_string());
    }

    println!("\nDocked ships:");
    for (i, ship) in docked_ships.iter().enumerate() {
        println!(
            "  {}. Ship #{} at Berth #{} - {}/{} containers remaining",
            i + 1,
            ship.id.0,
            ship.docked_at.unwrap().0,
            ship.containers_remaining,
            ship.containers
        );
    }

    print!("\nSelect ship number: ");
    io::stdout().flush().unwrap();
    let ship_idx = get_user_index()? - 1;

    if ship_idx >= docked_ships.len() {
        return Err("Invalid ship number".to_string());
    }

    let ship_id = docked_ships[ship_idx].id;

    // Show available cranes
    let free_cranes = port.free_cranes();
    if free_cranes.is_empty() {
        return Err("No cranes available!".to_string());
    }

    println!("\nAvailable cranes:");
    for (i, crane) in free_cranes.iter().enumerate() {
        println!(
            "  {}. Crane #{} (speed: {:.1})",
            i + 1,
            crane.id.0,
            crane.processing_speed
        );
    }

    print!("\nSelect crane number: ");
    io::stdout().flush().unwrap();
    let crane_idx = get_user_index()? - 1;

    if crane_idx >= free_cranes.len() {
        return Err("Invalid crane number".to_string());
    }

    let crane_id = free_cranes[crane_idx].id;

    Ok(PlayerAction::AssignCrane { crane_id, ship_id })
}

fn get_user_index() -> Result<usize, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    input
        .trim()
        .parse::<usize>()
        .map_err(|_| "Please enter a valid number".to_string())
}

/// Process player menu choice
pub fn process_player_choice(choice: u32, session: &GameSession) -> Result<PlayerAction, String> {
    match choice {
        1 => handle_dock_ship_input(&session.player_port),
        2 => handle_assign_crane_input(&session.player_port),
        3 => Ok(PlayerAction::ViewState),
        4 => Ok(PlayerAction::ViewComparison),
        5 => Ok(PlayerAction::EndTurn),
        6 => Ok(PlayerAction::Quit),
        _ => Err("Invalid choice. Please select 1-6.".to_string()),
    }
}
