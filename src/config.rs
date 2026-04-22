use crate::sessions::PuzzleType;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct TimerConfig {
    pub inspection_time: u64,
    pub hide_timer_while_solving: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    pub data_dir: PathBuf,
    pub default_puzzle: PuzzleType,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            data_dir: dirs::data_dir()
                .map(|d| d.join("lazytimer"))
                .unwrap_or(PathBuf::default()),
            default_puzzle: PuzzleType::ThreeByThree,
        }
    }
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self {
            inspection_time: 15,
            hide_timer_while_solving: false,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub timer: TimerConfig,
}

pub fn load_config() -> Result<Config> {
    confy::load("lazytimer", "config").context("Failed to load config")
}
