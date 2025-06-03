use anyhow::{Context, Result};
use std::time::{Duration, Instant};

use crate::{
    scramble::Scramble,
    sessions::{load_sessions, Session},
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

pub enum Screen {
    Timer,
    Statistics,
}

pub struct App {
    pub current_screen: Screen,
    pub exiting: bool,

    pub timer_state: TimerState,
    pub current_scramble: Scramble,

    pub sessions: Vec<Session>,
    pub current_session_idx: usize,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(App {
            current_screen: Screen::Timer,
            exiting: false,

            timer_state: TimerState::Idle {
                time: Duration::from_secs(0),
            },
            current_scramble: Scramble::new(),

            sessions: load_sessions().context("Failed to load sessions")?,
            current_session_idx: 0,
        })
    }

    pub fn toggle_screen(&mut self) {
        self.current_screen = match self.current_screen {
            Screen::Timer => Screen::Statistics,
            Screen::Statistics => Screen::Timer,
        };
    }

    pub fn current_session(&mut self) -> &mut Session {
        &mut self.sessions[self.current_session_idx]
    }
}
