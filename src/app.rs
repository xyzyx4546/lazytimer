use crate::{
    config::{Config, load_config},
    sessions::{PuzzleType, Solve},
};
use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use strum::IntoEnumIterator;

#[derive(Copy, Clone)]
pub enum TimerState {
    Idle { time: Duration },
    PreInspection { time: Duration },
    Inspection { start: Instant },
    PreRunning { start: Instant },
    Running { start: Instant },
}
pub enum PopupType {
    Keybinds,
    ConfirmDelete,
    SolveDetails,
}

pub struct App {
    pub config: Config,

    pub exiting: bool,
    pub popup: Option<PopupType>,

    pub timer_state: TimerState,
    pub current_scramble: String,
    pub selected_puzzle_type: PuzzleType,
    pub selected_solve_idx: usize,
    pub ghost_mode: bool,

    pub sessions: HashMap<PuzzleType, Vec<Solve>>,
    pub ao5: Vec<Option<Duration>>,
    pub ao12: Vec<Option<Duration>>,
}

impl Default for App {
    fn default() -> Self {
        let mut sessions = HashMap::new();
        for puzzle_type in PuzzleType::iter() {
            sessions.insert(puzzle_type, Vec::new());
        }

        Self {
            config: Config::default(),

            exiting: false,
            popup: None,

            timer_state: TimerState::Idle {
                time: Duration::from_secs(0),
            },
            current_scramble: String::new(),
            selected_puzzle_type: PuzzleType::ThreeByThree,
            selected_solve_idx: 0,
            ghost_mode: false,

            sessions,
            ao5: vec![],
            ao12: vec![],
        }
    }
}

impl App {
    pub fn new() -> Result<Self> {
        let mut app = Self::default();

        let config = load_config()?;

        app.selected_puzzle_type = config.general.default_puzzle;
        app.config = config;

        app.load_sessions().context("Failed to load sessions")?;
        app.reset_selected_solve();
        app.next_scramble();
        app.compute_averages();

        Ok(app)
    }

    pub fn selected_session(&self) -> &Vec<Solve> {
        self.sessions
            .get(&self.selected_puzzle_type)
            .expect("Selected puzzle type not found in sessions")
    }

    pub fn selected_session_mut(&mut self) -> &mut Vec<Solve> {
        self.sessions
            .get_mut(&self.selected_puzzle_type)
            .expect("Selected puzzle type not found in sessions")
    }

    pub fn selected_solve(&self) -> Option<&Solve> {
        self.selected_session().get(self.selected_solve_idx)
    }

    pub fn selected_solve_mut(&mut self) -> Option<&mut Solve> {
        let idx = self.selected_solve_idx;
        self.selected_session_mut().get_mut(idx)
    }

    pub fn reset_selected_solve(&mut self) {
        self.selected_solve_idx = self.selected_session().len().saturating_sub(1);
    }

    pub fn switch_solve(&mut self, offset: isize) {
        let new_idx = self.selected_solve_idx as isize + offset;

        if new_idx >= 0 && (new_idx as usize) < self.selected_session().len() {
            self.selected_solve_idx = new_idx as usize;
        }
    }

    pub fn switch_session(&mut self, offset: i32) {
        let puzzle_types: Vec<_> = PuzzleType::iter().collect();

        let current_idx = puzzle_types
            .iter()
            .position(|pt| pt == &self.selected_puzzle_type)
            .unwrap_or(0);
        let len = puzzle_types.len();
        let new_idx = ((current_idx as i32 + offset).rem_euclid(len as i32)) as usize;
        self.selected_puzzle_type = puzzle_types[new_idx];
        self.reset_selected_solve();
        self.next_scramble();
        self.compute_averages();
    }

    pub fn add_solve(&mut self, solve: Solve) {
        let session = self.selected_session_mut();
        session.push(solve);
        self.selected_solve_idx = session.len() - 1;
        self.compute_averages();
    }

    pub fn delete_solve(&mut self) {
        let idx = self.selected_solve_idx;
        self.selected_session_mut().remove(idx);
        self.selected_solve_idx = idx.saturating_sub(1);
        self.compute_averages();
    }
}
