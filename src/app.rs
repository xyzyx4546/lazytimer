use crate::sessions::{PuzzleType, Solve};
use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use strum::IntoEnumIterator;

pub const INSPECTION_TIME: u64 = 15;

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
    pub exiting: bool,

    pub timer_state: TimerState,
    pub current_scramble: String,

    pub sessions: HashMap<PuzzleType, Vec<Solve>>,
    pub selected_puzzle_type: PuzzleType,
    pub selected_solve_idx: usize,

    pub popup: Option<PopupType>,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut sessions = HashMap::new();
        for puzzle_type in PuzzleType::iter() {
            sessions.insert(puzzle_type, Vec::new());
        }

        let mut app = App {
            exiting: false,
            timer_state: TimerState::Idle {
                time: Duration::from_secs(0),
            },
            current_scramble: String::from(""),
            sessions,
            selected_puzzle_type: PuzzleType::ThreeByThree,
            selected_solve_idx: 0,
            popup: None,
        };

        app.load_sessions().context("Failed to load sessions")?;
        app.reset_selected_solve();
        app.next_scramble();

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

    pub fn add_solve(&mut self, solve: Solve) {
        let session = self.selected_session_mut();
        session.push(solve);
        self.selected_solve_idx = session.len() - 1;
    }

    pub fn reset_selected_solve(&mut self) {
        self.selected_solve_idx = self.selected_session().len().saturating_sub(1);
    }

    pub fn switch_session(&mut self, offset: i32) {
        let puzzle_types: Vec<_> = PuzzleType::iter().collect();

        let current_idx = puzzle_types
            .iter()
            .position(|pt| pt == &self.selected_puzzle_type)
            .unwrap_or(0);
        let len = puzzle_types.len();
        let new_idx = ((current_idx as i32 + offset).rem_euclid(len as i32)) as usize;
        self.selected_puzzle_type = puzzle_types[new_idx].clone();
        self.reset_selected_solve();
        self.next_scramble();
    }

    pub fn switch_solve(&mut self, offset: isize) {
        let new_idx = self.selected_solve_idx as isize + offset;

        if new_idx >= 0 && (new_idx as usize) < self.selected_session().len() {
            self.selected_solve_idx = new_idx as usize;
        }
    }
}
