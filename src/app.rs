use anyhow::{Context, Result};
use std::time::{Duration, Instant};

use crate::{
    scramble::Scramble,
    sessions::{load_sessions, Session, Solve},
};

pub const INSPECTION_TIME: u64 = 15;

#[derive(Clone)]
pub enum TimerState {
    Idle { time: Duration },
    PreInspection { time: Duration },
    Inspection { start: Instant },
    PreRunning { start: Instant },
    Running { start: Instant },
}

pub struct App {
    pub exiting: bool,

    pub timer_state: TimerState,
    pub current_scramble: Scramble,

    pub sessions: Vec<Session>,
    pub selected_session_idx: usize,
    pub selected_solve_idx: usize,
}

impl App {
    pub fn new() -> Result<Self> {
        let sessions = load_sessions().context("Failed to load sessions")?;

        let selected_solve_idx = sessions
            .get(0)
            .map(|session| session.solves.len().saturating_sub(1))
            .unwrap_or(0);

        Ok(App {
            exiting: false,
            timer_state: TimerState::Idle {
                time: Duration::from_secs(0),
            },
            current_scramble: Scramble::new(),
            sessions,
            selected_session_idx: 0,
            selected_solve_idx,
        })
    }

    pub fn selected_session(&mut self) -> &mut Session {
        &mut self.sessions[self.selected_session_idx]
    }

    pub fn selected_solve(&mut self) -> Option<&mut Solve> {
        let session = &mut self.sessions[self.selected_session_idx];
        session.solves.get_mut(self.selected_solve_idx)
    }

    pub fn add_solve(&mut self, solve: Solve) {
        let session = &mut self.sessions[self.selected_session_idx];
        session.solves.push(solve);
        self.selected_solve_idx = session.solves.len() - 1;
    }

    pub fn reset_selected_solve(&mut self) {
        self.selected_solve_idx = self.selected_session().solves.len().saturating_sub(1)
    }

    pub fn switch_session(&mut self, offset: i32) {
        let new_idx = self.selected_session_idx as i32 + offset;

        if new_idx >= 0 && (new_idx as usize) < self.sessions.len() {
            self.selected_session_idx = new_idx as usize;
            self.reset_selected_solve();
        }
    }

    pub fn switch_solve(&mut self, offset: isize) {
        let new_idx = self.selected_solve_idx as isize + offset;

        if new_idx >= 0 && (new_idx as usize) < self.selected_session().solves.len() {
            self.selected_solve_idx = new_idx as usize;
        }
    }
}
