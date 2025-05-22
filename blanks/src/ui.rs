use colored::Colorize;
use std::io::{self, Write};
use std::time::Duration;

use crate::game_state::GameState;

/// Display welcome message and game settings
pub fn display_welcome(game_state: &GameState) {
    println!("{}", "Welcome to Blanks: a Leetcode Game!".bright_green());
    println!("Language: {}, Difficulty: {}, Mode: {}", 
             game_state.config.language, 
             game_state.config.diff, 
             game_state.config.mode);
    println!("Press Ctrl+C to exit and see your score.\n");
}

/// Display information about the current problem
pub fn display_problem_info(problem_name: &str, blank_count: usize) {
    println!("Problem: {}", problem_name);
    println!("Blanks to fill: {}", blank_count);
    println!("{}", "Edit the file to fill in the blanks. Press Enter when done.".bright_cyan());
}

/// Wait for the user to press Enter
pub fn wait_for_enter() -> io::Result<()> {
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(())
}

/// Display the results of a round
pub fn display_round_results(
    correct_blanks: usize,
    total_blanks: usize,
    incorrect_chars: &[(usize, char, char)],
    elapsed: Duration
) {
    println!("Time taken: {:.2} seconds", elapsed.as_secs_f64());
    println!("Correct blanks: {}/{}", correct_blanks, total_blanks);
    
    if !incorrect_chars.is_empty() {
        println!("{}", "Incorrect characters:".bright_red());
        for &(idx, original, user) in incorrect_chars {
            println!("Position {}: Expected '{}', got '{}'", idx, original, user);
        }
    }
}

/// Display final score and statistics
pub fn display_score(game_state: &GameState) {
    println!("\n{}", "=== Game Over ===".bright_yellow());
    println!("Problems completed: {}", game_state.problems_completed);
    println!("Total time: {:.2} seconds", game_state.get_total_time().as_secs_f64());
    println!("Blanks correctly filled: {}/{}", 
             game_state.successfully_filled_blanks, 
             game_state.total_blanks);
    
    if game_state.total_blanks > 0 {
        let accuracy = game_state.get_accuracy();
        println!("Accuracy: {:.2}%", accuracy * 100.0);
        println!("Final score: {:.2}", game_state.calculate_score());
    }
}
