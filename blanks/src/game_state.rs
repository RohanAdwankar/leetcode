use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub config: Config,
    #[serde(skip)]
    pub current_problem: Option<String>,
    #[serde(skip)]
    pub original_content: Option<String>,
    #[serde(skip)]
    pub blanked_content: Option<String>,
    #[serde(skip)]
    pub start_time: Option<Instant>,
    pub problems_completed: usize,
    pub total_time_seconds: f64,
    pub successfully_filled_blanks: usize,
    pub total_blanks: usize,
}

impl GameState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            current_problem: None,
            original_content: None,
            blanked_content: None,
            start_time: None,
            problems_completed: 0,
            total_time_seconds: 0.0,
            successfully_filled_blanks: 0,
            total_blanks: 0,
        }
    }

    pub fn add_elapsed_time(&mut self) {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed();
            self.total_time_seconds += elapsed.as_secs_f64();
        }
    }

    pub fn get_total_time(&self) -> Duration {
        Duration::from_secs_f64(self.total_time_seconds)
    }

    pub fn get_accuracy(&self) -> f64 {
        if self.total_blanks == 0 {
            return 0.0;
        }
        self.successfully_filled_blanks as f64 / self.total_blanks as f64
    }

    pub fn calculate_score(&self) -> f64 {
        let accuracy = self.get_accuracy();
        let time_factor = if self.total_time_seconds > 0.0 {
            1.0 / self.total_time_seconds
        } else {
            1.0
        };
        
        (accuracy * 1000.0) * time_factor * self.problems_completed as f64
    }
}
