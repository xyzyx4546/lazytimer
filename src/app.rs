use anyhow::{Context, Result};
use std::time::{Duration, Instant};

use crate::sessions::{PuzzleType, Session, Solve};

pub const INSPECTION_TIME: u64 = 15;

#[derive(Copy, Clone)]
pub enum TimerState {
    Idle { time: Duration },
    PreInspection { time: Duration },
    Inspection { start: Instant },
    PreRunning { start: Instant },
    Running { start: Instant },
}

pub enum DeletionTarget {
    Solve,
    Session,
}

pub enum PopupType {
    Keybinds,
    ConfirmDelete {
        target: DeletionTarget,
    },
    SolveDetails,
    CreateSession {
        name_buffer: String,
        selected_puzzle_type: PuzzleType,
    },
}

pub struct App {
    pub exiting: bool,

    pub timer_state: TimerState,
    pub current_scramble: String,

    pub sessions: Vec<Session>,
    pub selected_session_idx: usize,
    pub selected_solve_idx: usize,

    pub popup: Option<PopupType>,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut app = App {
            exiting: false,
            timer_state: TimerState::Idle {
                time: Duration::from_secs(0),
            },
            current_scramble: String::from(""),
            sessions: vec![],
            selected_session_idx: 0,
            selected_solve_idx: 0,
            popup: None,
        };

        app.load_sessions().context("Failed to load sessions")?;

        if !app.sessions.is_empty() {
            app.reset_selected_solve();
            app.next_scramble();
        }

        Ok(app)
    }

    pub fn selected_session(&self) -> &Session {
        &self.sessions[self.selected_session_idx]
    }

    pub fn selected_session_mut(&mut self) -> &mut Session {
        &mut self.sessions[self.selected_session_idx]
    }

    pub fn selected_solve(&self) -> Option<&Solve> {
        self.selected_session().solves.get(self.selected_solve_idx)
    }

    pub fn selected_solve_mut(&mut self) -> Option<&mut Solve> {
        let idx = self.selected_solve_idx;
        self.selected_session_mut().solves.get_mut(idx)
    }

    pub fn add_solve(&mut self, solve: Solve) {
        let session = &mut self.sessions[self.selected_session_idx];
        session.solves.push(solve);
        self.selected_solve_idx = session.solves.len() - 1;
    }

    pub fn reset_selected_solve(&mut self) {
        self.selected_solve_idx = self.selected_session().solves.len().saturating_sub(1);
    }

    pub fn switch_session(&mut self, offset: i32) {
        let new_idx = self.selected_session_idx as i32 + offset;

        if new_idx >= 0 && (new_idx as usize) < self.sessions.len() {
            self.selected_session_idx = new_idx as usize;
            self.reset_selected_solve();
            self.next_scramble();
        }
    }

    pub fn switch_solve(&mut self, offset: isize) {
        let new_idx = self.selected_solve_idx as isize + offset;

        if new_idx >= 0 && (new_idx as usize) < self.selected_session().solves.len() {
            self.selected_solve_idx = new_idx as usize;
        }
    }

    pub fn add_session(&mut self, name: String, puzzle_type: PuzzleType) {
        self.sessions.push(Session {
            name,
            puzzle_type,
            solves: vec![],
        });
        self.selected_session_idx = self.sessions.len() - 1;
        self.reset_selected_solve();
        self.next_scramble();
    }
}
