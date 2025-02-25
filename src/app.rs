#![allow(dead_code)]
use std::time::{Duration, Instant};

use crate::scramble::generate_scramble;

pub const INSPECTION_TIME: u64 = 15;

#[derive(Clone, PartialEq)]
pub enum Penalty {
    None,
    PlusTwo,
    Dnf,
}

impl Penalty {
    pub fn toggle(&mut self, penalty: Penalty) {
        *self = if *self == penalty {
            Penalty::None
        } else {
            penalty
        };
    }
}

#[derive(Clone)]
pub struct Solve {
    pub time: Duration,
    pub penalty: Penalty,
    pub scramble: String,
    pub timestamp: Instant,
}

pub struct Session {
    pub name: String,
    pub solves: Vec<Solve>,
}

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
    pub show_help: bool,
    pub exiting: bool,

    pub timer_state: TimerState,
    pub current_scramble: String,

    pub current_session: Session,

    pub selected_solve: usize,
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: Screen::Timer,
            show_help: false,
            exiting: false,
            timer_state: TimerState::Idle {
                time: Duration::from_secs(0),
            },
            current_scramble: generate_scramble(),
            current_session: Session {
                name: String::from("test Session"),
                solves: vec![],
            },
            selected_solve: 0,
        }
    }

    pub fn toggle_screen(&mut self) {
        self.current_screen = match self.current_screen {
            Screen::Timer => Screen::Statistics,
            Screen::Statistics => Screen::Timer,
        };
    }

    pub fn add_solve(&mut self, duration: Duration) {
        self.current_session.solves.push(Solve {
            time: duration,
            penalty: Penalty::None,
            scramble: self.current_scramble.clone(),
            timestamp: Instant::now(),
        });
    }
}
