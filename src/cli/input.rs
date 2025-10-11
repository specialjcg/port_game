// Input utilities for CLI

use std::io::{self, Write};

/// Get yes/no confirmation from user
pub fn confirm(prompt: &str) -> bool {
    print!("{} (y/n): ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

/// Wait for user to press Enter
pub fn wait_for_enter() {
    println!("\nPress Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();
}

/// Clear screen (works on Unix-like systems)
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}
