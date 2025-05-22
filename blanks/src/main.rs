mod config;
mod game_state;
mod problem;
mod ui;

use clap::Parser;
use std::fs;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about = "Leetcode game where you fill in blanks in code", long_about = None)]
struct Args {
    /// Programming language (defaults to python)
    #[arg(long)]
    language: Option<String>,

    /// Proportion of characters to remove (0.0-1.0)
    #[arg(long)]
    diff: Option<f64>,

    /// Problem number to start with
    #[arg(long)]
    problem: Option<usize>,

    /// Mode: 'random' or 'sequential'
    #[arg(long)]
    mode: Option<String>,
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();
    
    // Load previous config if it exists, or use defaults with command line overrides
    let mut config = config::load_config();
    
    // Override with command-line arguments if provided
    if let Some(lang) = args.language {
        config.language = lang;
    }
    
    if let Some(diff) = args.diff {
        let clamped_diff = diff.max(0.0).min(1.0);
        config.diff = clamped_diff;
    }
    
    if let Some(problem) = args.problem {
        config.problem = Some(problem);
    }
    
    if let Some(mode) = args.mode {
        if mode == "random" || mode == "sequential" {
            config.mode = mode;
        }
    }
    
    // Create game state with the config
    let mut game_state = game_state::GameState::new(config.clone());
    
    // Set up ctrl+c handler
    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);
    
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
    
    // Display welcome message
    ui::display_welcome(&game_state);
    
    // Main game loop
    while running.load(Ordering::SeqCst) {
        match play_round(&mut game_state) {
            Ok(_) => {
                println!("\nRound complete! Moving to next problem...");
                game_state.problems_completed += 1;
                
                // Save config for future runs
                if let Err(e) = config::save_config(&game_state.config) {
                    eprintln!("Failed to save config: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Error during gameplay: {}", e);
                break;
            }
        }
    }
    
    // Clean up and display final score
    restore_original_file(&game_state);
    ui::display_score(&game_state);
}

fn play_round(game_state: &mut game_state::GameState) -> Result<(), io::Error> {
    // Clean up from previous round if necessary
    restore_original_file(game_state);
    
    // Get problem files for the selected language
    let problem_files = problem::get_problem_files(&game_state.config.language)?;
    
    if problem_files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("No problem files found for language: {}", game_state.config.language),
        ));
    }
    
    // Select a problem
    let problem_path = problem::select_problem(
        game_state.current_problem.as_deref(),
        game_state.config.problem,
        &game_state.config.mode,
        &problem_files
    ).ok_or_else(|| io::Error::new(
        io::ErrorKind::NotFound,
        "Failed to select a problem",
    ))?;
    
    // Read the original content
    let original_content = fs::read_to_string(&problem_path)?;
    
    // Create blanked content
    let (blanked_content, original_chars) = problem::create_blanked_content(
        &original_content, 
        game_state.config.diff
    );
    
    // Save state
    game_state.current_problem = Some(problem_path.to_string_lossy().to_string());
    game_state.original_content = Some(original_content.clone());
    game_state.blanked_content = Some(blanked_content.clone());
    game_state.start_time = Some(Instant::now());
    game_state.total_blanks += original_chars.len();
    
    // Write blanked content to file
    fs::write(&problem_path, &blanked_content)?;
    
    // Display problem information
    if let Some(file_name) = problem_path.file_name() {
        ui::display_problem_info(
            &file_name.to_string_lossy(), 
            original_chars.len()
        );
    }
    
    // Wait for user to press Enter
    ui::wait_for_enter()?;
    
    // Read the user's solution
    let user_solution = fs::read_to_string(&problem_path)?;
    
    // Calculate elapsed time
    let elapsed = if let Some(start_time) = game_state.start_time.take() {
        let elapsed = start_time.elapsed();
        game_state.total_time_seconds += elapsed.as_secs_f64();
        elapsed
    } else {
        std::time::Duration::from_secs(0)
    };
    
    // Check the solution
    let (correct_blanks, incorrect_chars) = problem::check_solution(&user_solution, &original_chars);
    
    game_state.successfully_filled_blanks += correct_blanks;
    
    // Show results
    ui::display_round_results(
        correct_blanks, 
        original_chars.len(), 
        &incorrect_chars,
        elapsed
    );
    
    // Restore original file
    restore_original_file(game_state);
    
    Ok(())
}

fn restore_original_file(game_state: &game_state::GameState) {
    if let (Some(problem_path), Some(original_content)) = (&game_state.current_problem, &game_state.original_content) {
        if let Err(e) = problem::restore_file(problem_path, original_content) {
            eprintln!("Failed to restore original file: {}", e);
        }
    }
}
