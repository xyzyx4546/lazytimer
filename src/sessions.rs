use crate::app::App;
use anyhow::{Context, Result};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{create_dir_all, read_to_string, write},
    time::{Duration, SystemTime},
};
use strum::EnumIter;

#[derive(Hash, EnumIter, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum PuzzleType {
    TwoByTwo,
    ThreeByThree,
    FourByFour,
    FiveByFive,
    Skewb,
    Pyraminx,
}

impl PuzzleType {
    pub fn to_string(&self) -> &str {
        match self {
            PuzzleType::TwoByTwo => "2x2",
            PuzzleType::ThreeByThree => "3x3",
            PuzzleType::FourByFour => "4x4",
            PuzzleType::FiveByFive => "5x5",
            PuzzleType::Skewb => "Skewb",
            PuzzleType::Pyraminx => "Pyraminx",
        }
    }
}

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

    pub fn toggle_penalty(&mut self, penalty: Penalty) {
        self.penalty = if self.penalty == penalty {
            Penalty::None
        } else {
            penalty
        };
    }
}

impl App {
    pub fn best_time(&self) -> Option<Duration> {
        self.selected_session()
            .iter()
            .filter_map(|solve| solve.effective_time())
            .min()
    }

    pub fn worst_time(&self) -> Option<Duration> {
        self.selected_session()
            .iter()
            .filter_map(|solve| solve.effective_time())
            .max()
    }

    pub fn ao(&self, k: usize) -> Vec<Option<Duration>> {
        if k < 3 {
            return vec![None; self.selected_session().len()];
        }
        (0..self.selected_session().len())
            .map(|i| {
                if i < k - 1 {
                    return None;
                }
                let start = i + 1 - k;
                let times: Vec<Option<Duration>> = self.selected_session()[start..=i]
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

    pub fn load_sessions(&mut self) -> Result<()> {
        let config_path = config_dir()
            .context("Config directory not found")?
            .join("lazytimer/sessions.json");

        if config_path.exists() {
            let json = read_to_string(&config_path).context("Failed to read sessions file")?;
            let sessions: HashMap<PuzzleType, Vec<Solve>> =
                serde_json::from_str(&json).context("Failed to parse sessions JSON")?;
            self.sessions = sessions;
        }
        Ok(())
    }

    pub fn save_sessions(&self) -> Result<()> {
        let path = config_dir()
            .context("Config directory not found")?
            .join("lazytimer/sessions.json");
        create_dir_all(path.parent().context("Path has no parent")?)?;
        let json =
            serde_json::to_string_pretty(&self.sessions).context("Failed to serialize sessions")?;
        write(&path, json).context("Failed to write sessions file")?;
        Ok(())
    }
}
