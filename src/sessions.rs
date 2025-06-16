#![allow(unused)]

use anyhow::{Context, Result};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, read_to_string, write};
use std::time::{Duration, SystemTime};

use crate::app::App;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Penalty {
    None,
    PlusTwo,
    Dnf,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Solve {
    pub time: Duration,
    pub penalty: Penalty,
    pub scramble: String,
    pub timestamp: SystemTime,
}

impl Solve {
    pub fn effective_time(&self) -> Option<Duration> {
        match self.penalty {
            Penalty::None => Some(self.time),
            Penalty::PlusTwo => Some(self.time + Duration::from_secs(2)),
            Penalty::Dnf => None,
        }
    }

    pub fn toggle_panalty(&mut self, penalty: Penalty) {
        self.penalty = if self.penalty == penalty {
            Penalty::None
        } else {
            penalty
        };
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum PuzzleType {
    ThreeByThree,
    TwoByTwo,
}

impl PuzzleType {
    pub fn to_string(&self) -> &str {
        match self {
            PuzzleType::ThreeByThree => "3x3",
            PuzzleType::TwoByTwo => "2x2",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    pub puzzle_type: PuzzleType,
    pub solves: Vec<Solve>,
}

impl Session {
    pub fn new(name: String, puzzle_type: PuzzleType) -> Self {
        Self {
            name,
            puzzle_type,
            solves: vec![],
        }
    }
    pub fn best_time(&self) -> Option<Duration> {
        self.solves
            .iter()
            .filter_map(|solve| solve.effective_time())
            .min()
    }

    pub fn worst_time(&self) -> Option<Duration> {
        self.solves
            .iter()
            .filter_map(|solve| solve.effective_time())
            .max()
    }

    pub fn ao(&self, k: usize) -> Vec<Option<Duration>> {
        if k < 3 {
            return vec![None; self.solves.len()];
        }
        (0..self.solves.len())
            .map(|i| {
                if i < k - 1 {
                    return None;
                }
                let start = i + 1 - k;
                let times: Vec<Option<Duration>> = self.solves[start..=i]
                    .iter()
                    .map(|s| s.effective_time())
                    .collect();
                let num_none = times.iter().filter(|t| t.is_none()).count();
                if num_none > 1 {
                    return None;
                }
                let mut some_times: Vec<Duration> = times.iter().filter_map(|t| *t).collect();
                if some_times.len() < k - 1 {
                    return None;
                }
                some_times.sort();
                let to_average = &some_times[1..k - 1];
                let total = to_average.iter().sum::<Duration>();
                Some(total / to_average.len() as u32)
            })
            .collect()
    }
}

impl App {
    pub fn load_sessions(&mut self) -> Result<()> {
        let config_path = config_dir()
            .context("Config directory not found")?
            .join("lazytimer/sessions.json");

        if config_path.exists() {
            let json = read_to_string(&config_path).context("Failed to read sessions file")?;
            let sessions: Vec<Session> =
            serde_json::from_str(&json).context("Failed to parse sessions JSON")?;
            self.sessions = sessions;
        } else {
            let default_sessions = vec![
                Session {
                    name: String::from("Session #1"),
                    puzzle_type: PuzzleType::ThreeByThree,
                    solves: vec![],
                },
                Session {
                    name: String::from("Session #2"),
                    puzzle_type: PuzzleType::TwoByTwo,
                    solves: vec![],
                },
                Session {
                    name: String::from("Session #3"),
                    puzzle_type: PuzzleType::ThreeByThree,
                    solves: vec![],
                },
            ];
            self.save_sessions().context("Failed to save default sessions")?;
            self.sessions = default_sessions;
        }
        Ok(())
    }

    pub fn save_sessions(&self) -> Result<()> {
        let path = config_dir()
            .context("Config directory not found")?
            .join("lazytimer/sessions.json");
        create_dir_all(path.parent().context("Path has no parent")?)?;
        let json = serde_json::to_string_pretty(&self.sessions).context("Failed to serialize sessions")?;
        write(&path, json).context("Failed to write sessions file")?;
        Ok(())
    }
}

