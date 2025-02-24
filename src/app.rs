#![allow(dead_code)]
use std::time::{Duration, Instant};

use crate::scramble::generate_scramble;

pub const INSPECTION_TIME: u64 = 15;

pub enum Penalty {
    None,
    PlusTwo,
    Dnf,
}

pub struct Solve {
    time: Duration,
    penalty: Penalty,
    scramble: String,
    timestamp: Instant,
}

pub struct Session {
    name: String,
    solves: Vec<Solve>,
}

#[derive(Clone)]
pub enum TimerState {
    Idle {
        time: Duration,
    },
    PreInspection {
        time: Duration,
    },
    Inspection {
        start: Instant,
    },
    PreRunning {
        start: Instant,
    },
    Running {
        start: Instant,
    },
}

pub enum Screen {
    Timer,
    Statistics,
}

pub struct App {
    pub current_screen: Screen,
    pub show_help: bool,
    pub exiting: bool,

    pub timer_state: TimerState,
    pub current_scramble: String,

    pub current_session: usize,

    pub selected_solve: usize,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: Screen::Timer,
            show_help: false,
            exiting: false,
            timer_state: TimerState::Idle { time: Duration::from_secs(0) },
            current_scramble: generate_scramble(),
            current_session: 0,
            selected_solve: 0,
        }
    }

    pub fn toggle_screen(&mut self) {
        self.current_screen = match self.current_screen {
            Screen::Timer => Screen::Statistics,
            Screen::Statistics => Screen::Timer,
        };
    }
}
