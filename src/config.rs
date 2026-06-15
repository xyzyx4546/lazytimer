use crate::sessions::PuzzleType;
use anyhow::{Context, Result};
use crokey::{KeyCombination, key};
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

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct KeybindsConfig {
    pub previous_puzzle: KeyCombination,
    pub next_puzzle: KeyCombination,
    pub previous_solve: KeyCombination,
    pub next_solve: KeyCombination,
    pub first_solve: KeyCombination,
    pub last_solve: KeyCombination,

    pub quit: KeyCombination,
    pub show_keybinds: KeyCombination,
    pub cancel: KeyCombination,
    pub confirm: KeyCombination,
    pub start_timer: KeyCombination,
    pub toggle_ghost_mode: KeyCombination,

    pub delete_solve: KeyCombination,
    pub solve_details: KeyCombination,
    pub toggle_plus_two: KeyCombination,
    pub toggle_dnf: KeyCombination,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            data_dir: dirs::data_dir()
                .map(|d| d.join("lazytimer"))
                .unwrap_or_default(),
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

impl Default for KeybindsConfig {
    fn default() -> Self {
        Self {
            previous_puzzle: key!(h),
            next_puzzle: key!(l),
            previous_solve: key!(j),
            next_solve: key!(k),
            first_solve: key!(g),
            last_solve: key!(shift - g),

            quit: key!(q),
            show_keybinds: key!('?'),
            cancel: key!(esc),
            confirm: key!(enter),
            start_timer: key!(space),
            toggle_ghost_mode: key!(v),

            delete_solve: key!(d),
            solve_details: key!(i),
            toggle_plus_two: key!('+'),
            toggle_dnf: key!('-'),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub general: GeneralConfig,
    pub timer: TimerConfig,
    pub keybinds: KeybindsConfig,
}

pub fn load_config() -> Result<Config> {
    confy::load("lazytimer", "config").context("Failed to load config")
}
