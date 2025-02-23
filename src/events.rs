use crate::app::{INSPECTION_TIME, App, TimerState};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io::Result;
use std::time::{Duration, Instant};

fn handle_input(app: &mut App, code: KeyCode, kind: KeyEventKind) {
    if let TimerState::Running { start } = app.timer_state {
        app.timer_state = TimerState::Idle { time: start.elapsed() };
        return;
    }
    match (code, kind) {
        (KeyCode::Char('q'), KeyEventKind::Press) => app.exiting = true,
        (KeyCode::Tab, KeyEventKind::Press) => app.toggle_screen(),

        (KeyCode::Char(' '), KeyEventKind::Press) => {
            app.timer_state = match app.timer_state {
                TimerState::Idle { time } => TimerState::PreInspection { time },
                TimerState::Inspection { start } => TimerState::PreRunning { start },
                TimerState::Running { start } => TimerState::Idle {
                    time: start.elapsed(),
                },
                _ => app.timer_state.clone(),
            };
        }
        (KeyCode::Char(' '), KeyEventKind::Release) => {
            app.timer_state = match app.timer_state {
                TimerState::PreInspection { .. } => TimerState::Inspection {
                    start: Instant::now(),
                },
                TimerState::PreRunning { .. } => TimerState::Running {
                    start: Instant::now(),
                },
                _ => app.timer_state.clone(),
            };
        }
        _ => {}
    }
}

pub fn handle(app: &mut App) -> Result<()> {
    if let TimerState::Inspection { start } = app.timer_state {
        if INSPECTION_TIME - start.elapsed().as_secs() == 0 {
            app.timer_state = TimerState::Running {
                start: Instant::now(),
            };
        }
    }

    let poll_time = Duration::from_millis(match app.timer_state {
        TimerState::Idle { .. } | TimerState::PreInspection { .. } => 1000,
        TimerState::PreRunning { .. } | TimerState::Inspection { .. } => 200,
        TimerState::Running { .. } => 16,
    });

    if poll(poll_time)? {
        if let Event::Key(KeyEvent { code, kind, .. }) = read()? {
            handle_input(app, code, kind);
        }
    }
    Ok(())
}
