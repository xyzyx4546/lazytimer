#![allow(dead_code)]
use std::time::{Duration, Instant};

use crate::scramble::Scramble;

pub const INSPECTION_TIME: u64 = 15;

#[derive(Clone, PartialEq)]
pub enum Penalty {
    None,
    PlusTwo,
    Dnf,
}

#[derive(Clone)]
pub struct Solve {
    pub time: Duration,
    pub penalty: Penalty,
    pub scramble: Scramble,
    pub timestamp: Instant,
}

impl Solve {
    fn effective_time(&self) -> Option<Duration> {
        match self.penalty {
            Penalty::None => Some(self.time),
            Penalty::PlusTwo => Some(self.time + Duration::from_secs(2)),
            Penalty::Dnf => None,
        }
    }

    pub fn toggle_panalty(&mut self, penalty: Penalty) {
        self.penalty = if self.penalty == penalty {
            Penalty::None
        } else {
            penalty
        };
    }
}

// TODO: put sessions in its own module
pub struct Session {
    pub name: String,
    pub solves: Vec<Solve>,
}

impl Session {
    pub fn best_time(&self) -> Option<Duration> {
        self.solves
            .iter()
            .filter_map(|solve| solve.effective_time())
            .min()
    }

    pub fn calculate_average(&self, num_solves: usize) -> Option<Duration> {
        if self.solves.len() < num_solves {
            return None;
        }

        let latest_solves = &self.solves[self.solves.len() - num_solves..];
        let dnf_count = latest_solves
            .iter()
            .filter(|s| s.penalty == Penalty::Dnf)
            .count();
        if dnf_count > 1 {
            return None;
        }

        let mut times: Vec<_> = latest_solves.iter().map(|s| s.effective_time()).collect();
        times.sort_by(|a, b| match (a, b) {
            (Some(a), Some(b)) => a.cmp(b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });

        let remaining = &times[1..times.len() - 1];
        let total = remaining.iter().copied().flatten().sum::<Duration>();
        Some(total / remaining.len() as u32)
    }
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
    pub current_scramble: Scramble,

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
            current_scramble: Scramble::new(),
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
}
