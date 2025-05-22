use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The character used to represent blanks in the code
pub const BLANK_CHAR: char = '_';

/// Get all problem files for the specified language
pub fn get_problem_files(language: &str) -> Result<Vec<PathBuf>, io::Error> {
    let language_dir = Path::new("/Users/rohanadwankar/leet").join(language);
    
    if !language_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Language directory not found: {}", language_dir.display()),
        ));
    }
    
    let mut problem_files = Vec::new();
    for entry in fs::read_dir(language_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            problem_files.push(path);
        }
    }
    
    Ok(problem_files)
}

/// Select a problem based on the game configuration
pub fn select_problem(
    current_problem: Option<&str>,
    problem_number: Option<usize>,
    mode: &str,
    problem_files: &[PathBuf]
) -> Option<PathBuf> {
    if problem_files.is_empty() {
        return None;
    }
    
    if let Some(problem_num) = problem_number {
        // Try to find a specific problem by number
        for path in problem_files {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if file_name_str.starts_with(&format!("{:04}", problem_num)) {
                        return Some(path.clone());
                    }
                }
            }
        }
        
        // If specific problem not found, fall back to random or sequential
        println!("Problem {} not found, falling back to {}", problem_num, mode);
    }
    
    match mode {
        "random" => {
            let mut rng = rand::thread_rng();
            problem_files.choose(&mut rng).cloned()
        },
        "sequential" => {
            // In sequential mode, choose the next problem after the last one
            if let Some(current_problem_path) = current_problem {
                let current_path = Path::new(current_problem_path);
                let current_idx = problem_files.iter()
                    .position(|p| p == current_path)
                    .unwrap_or(0);
                let next_idx = (current_idx + 1) % problem_files.len();
                Some(problem_files[next_idx].clone())
            } else {
                // If no current problem, start with the first one
                problem_files.first().cloned()
            }
        },
        _ => Some(problem_files[0].clone()), // Default to first problem
    }
}

/// Create blanked content with the specified difficulty
pub fn create_blanked_content(content: &str, diff: f64) -> (String, HashMap<usize, char>) {
    let mut rng = rand::thread_rng();
    let mut blanked_content = content.chars().collect::<Vec<char>>();
    let mut original_chars = HashMap::new();
    
    // Only replace alphanumeric and some special characters
    let replaceable_indices: Vec<usize> = blanked_content.iter().enumerate()
        .filter(|(_, c)| c.is_alphanumeric() || **c == '=' || **c == '+' || **c == '-' || **c == '*' || **c == '/' || **c == '%')
        .map(|(i, _)| i)
        .collect();
    
    let num_blanks = (replaceable_indices.len() as f64 * diff) as usize;
    let indices_to_blank: Vec<usize> = replaceable_indices.choose_multiple(&mut rng, num_blanks).cloned().collect();
    
    for &idx in &indices_to_blank {
        original_chars.insert(idx, blanked_content[idx]);
        blanked_content[idx] = BLANK_CHAR;
    }
    
    (blanked_content.iter().collect(), original_chars)
}

/// Check a user's solution against the original content
pub fn check_solution(
    user_solution: &str, 
    original_chars: &HashMap<usize, char>
) -> (usize, Vec<(usize, char, char)>) {
    let mut correct_blanks = 0;
    let mut incorrect_chars = Vec::new();
    
    for (&idx, &original_char) in original_chars {
        if idx < user_solution.chars().count() {
            let user_char = user_solution.chars().nth(idx).unwrap();
            if user_char == original_char {
                correct_blanks += 1;
            } else {
                incorrect_chars.push((idx, original_char, user_char));
            }
        }
    }
    
    (correct_blanks, incorrect_chars)
}

/// Restore a file to its original content
pub fn restore_file(problem_path: &str, original_content: &str) -> Result<(), io::Error> {
    fs::write(problem_path, original_content)
}
