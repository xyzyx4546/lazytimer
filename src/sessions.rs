use anyhow::{Context, Result};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use std::fs::{create_dir_all, read_to_string, write};

use crate::scramble::Scramble;

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Penalty {
    None,
    PlusTwo,
    Dnf,
}

#[derive(Serialize, Deserialize)]
pub struct Solve {
    pub time: Duration,
    pub penalty: Penalty,
    pub scramble: Scramble,
    pub timestamp: SystemTime,
}

impl Solve {
    fn effective_time(&self) -> Option<Duration> {
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

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    pub solves: Vec<Solve>,
}

impl Session {
    pub fn best_time(&self) -> Option<Duration> {
        self.solves
            .iter()
            .filter_map(|solve| solve.effective_time())
            .min()
    }

    pub fn calculate_average(&self, num_solves: usize) -> Option<Duration> {
        if self.solves.len() < num_solves {
            return None;
        }

        let latest_solves = &self.solves[self.solves.len() - num_solves..];
        let dnf_count = latest_solves
            .iter()
            .filter(|s| s.penalty == Penalty::Dnf)
            .count();
        if dnf_count > 1 {
            return None;
        }

        let mut times: Vec<_> = latest_solves.iter().map(|s| s.effective_time()).collect();
        times.sort_by(|a, b| match (a, b) {
            (Some(a), Some(b)) => a.cmp(b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });

        let remaining = &times[1..times.len() - 1];
        let total = remaining.iter().copied().flatten().sum::<Duration>();
        Some(total / remaining.len() as u32)
    }
}

pub fn load_sessions() -> Result<Vec<Session>> {
    let config_path = config_dir()
        .context("Config directory not found")?
        .join("lazytimer/sessions.json");

    if config_path.exists() {
        let json = read_to_string(&config_path).context("Failed to read sessions file")?;
        let sessions: Vec<Session> = serde_json::from_str(&json).context("Failed to parse sessions JSON")?;
        Ok(sessions)
    } else {
        let default_sessions = vec![
            Session {
                name: String::from("Session #1"),
                solves: vec![],
            },
            Session {
                name: String::from("Session #2"),
                solves: vec![],
            },
            Session {
                name: String::from("Session #3"),
                solves: vec![],
            },
        ];
        save_sessions(&default_sessions).context("Failed to save default sessions")?;
        Ok(default_sessions)
    }
}

pub fn save_sessions(sessions: &[Session]) -> Result<()> {
    let path = config_dir()
        .context("Config directory not found")?
        .join("lazytimer/sessions.json");
    create_dir_all(path.parent().context("Path has no parent")?)?;
    let json = serde_json::to_string_pretty(sessions).context("Failed to serialize sessions")?;
    write(&path, json).context("Failed to write sessions file")?;
    Ok(())
}
