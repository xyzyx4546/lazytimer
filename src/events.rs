use crate::app::{App, Penalty, Screen, Solve, TimerState, INSPECTION_TIME};
use crate::scramble::Scramble;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io::Result;
use std::time::{Duration, Instant};

fn handle_key(app: &mut App, code: KeyCode, kind: KeyEventKind) {
    if let TimerState::Running { start } = app.timer_state {
        let time = start.elapsed();
        app.timer_state = TimerState::Idle { time };
        app.current_session.solves.push(Solve {
            time,
            penalty: Penalty::None,
            scramble: app.current_scramble.clone(),
            timestamp: Instant::now(),
        });
        app.current_scramble = Scramble::new();
        return;
    }

    if matches!(code, KeyCode::Char(' ')) {
        match kind {
            KeyEventKind::Press => {
                app.timer_state = match app.timer_state {
                    TimerState::Idle { time } => TimerState::PreInspection { time },
                    TimerState::Inspection { start } => TimerState::PreRunning { start },
                    TimerState::Running { start } => TimerState::Idle {
                        time: start.elapsed(),
                    },
                    _ => app.timer_state.clone(),
                };
            }
            KeyEventKind::Release => {
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

    if matches!(kind, KeyEventKind::Release) {
        return;
    }

    match code {
        KeyCode::Char('q') => app.exiting = true,
        KeyCode::Tab => app.toggle_screen(),

        KeyCode::Char('+') => {
            if matches!(app.current_screen, Screen::Timer)
                && matches!(app.timer_state, TimerState::Idle { .. })
            {
                if let Some(solve) = app.current_session.solves.last_mut() {
                    solve.toggle_panalty(Penalty::PlusTwo);
                }
            }
        }
        KeyCode::Char('-') => {
            if matches!(app.current_screen, Screen::Timer)
                && matches!(app.timer_state, TimerState::Idle { .. })
            {
                if let Some(solve) = app.current_session.solves.last_mut() {
                    solve.toggle_panalty(Penalty::Dnf);
                }
            }
        }
        _ => {}
    }
}

pub fn handle(app: &mut App) -> Result<()> {
    if let TimerState::Inspection { start } = app.timer_state {
        if INSPECTION_TIME - start.elapsed().as_secs() <= 0 {
            app.timer_state = TimerState::Running {
                start: Instant::now(),
            };
        }
    }

    let poll_time = Duration::from_millis(match app.timer_state {
        TimerState::Running { .. } => 100,
        _ => 1000,
    });

    if poll(poll_time)? {
        if let Event::Key(KeyEvent { code, kind, .. }) = read()? {
            handle_key(app, code, kind);
        }
    }
    Ok(())
}
