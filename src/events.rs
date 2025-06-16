use anyhow::{Context, Result};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};
use std::time::{Duration, Instant, SystemTime};

use crate::app::{App, PopupType, TimerState, INSPECTION_TIME};
use crate::sessions::{Penalty, Solve};

pub fn handle_space(app: &mut App, kind: KeyEventKind) -> Result<()> {
    match kind {
        KeyEventKind::Press => {
            app.timer_state = match app.timer_state {
                TimerState::Idle { time } => TimerState::PreInspection { time },
                TimerState::Inspection { start } => TimerState::PreRunning { start },
                TimerState::Running { start } => TimerState::Idle {
                    time: start.elapsed(),
                },
                _ => app.timer_state,
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
                _ => app.timer_state,
            };
        }
        _ => {}
    };
    Ok(())
}

pub fn handle_key(app: &mut App, code: KeyCode) -> Result<()> {
    if let Some(popup_type) = &app.popup {
        if matches!(code, KeyCode::Esc) {
            app.popup = None;
            return Ok(());
        }
        match popup_type {
            PopupType::ConfirmDelete => {
                if matches!(code, KeyCode::Enter) {
                    let index = app.selected_solve_idx;
                    app.selected_solve_idx = app.selected_solve_idx.saturating_sub(1);
                    app.selected_session().solves.remove(index);
                    app.popup = None;
                }
            }
            _ => {}
        }
    } else {
        match code {
            KeyCode::Char('q') => {
                app.save_sessions().context("Failed to save sessions")?;
                app.exiting = true;
            }
            KeyCode::Char('?') => app.popup = Some(PopupType::Keybinds),
            KeyCode::Char('d') => app.popup = Some(PopupType::ConfirmDelete),
            KeyCode::Char('i') => {
                if let Some(_) = app.selected_solve() {
                    app.popup = Some(PopupType::SolveDetails);
                }
            }
            KeyCode::Esc => app.popup = None,

            KeyCode::Char('h') | KeyCode::Left => app.switch_session(-1),
            KeyCode::Char('j') | KeyCode::Down => app.switch_solve(-1),
            KeyCode::Char('k') | KeyCode::Up => app.switch_solve(1),
            KeyCode::Char('l') | KeyCode::Right => app.switch_session(1),
            KeyCode::Char('g') => app.reset_selected_solve(),
            KeyCode::Char('G') => app.selected_solve_idx = 0,

            KeyCode::Char('+') => {
                if let Some(solve) = app.selected_solve() {
                    solve.toggle_panalty(Penalty::PlusTwo);
                }
            }
            KeyCode::Char('-') => {
                if let Some(solve) = app.selected_solve() {
                    solve.toggle_panalty(Penalty::Dnf);
                }
            }
            _ => {}
        };
    }
    Ok(())
}

pub fn handle(app: &mut App) -> Result<()> {
    if let TimerState::Inspection { start } = app.timer_state {
        if INSPECTION_TIME - start.elapsed().as_secs() <= 0 {
            app.timer_state = TimerState::Running {
                start: Instant::now(),
            };
        }
    }

    if matches!(app.timer_state, TimerState::Idle { .. }) || poll(Duration::from_millis(100))? {
        if let Event::Key(KeyEvent { code, kind, .. }) = read()? {
            if let TimerState::Running { start } = app.timer_state {
                let time = start.elapsed();
                app.timer_state = TimerState::Idle { time };
                let scramble = app.current_scramble.clone();
                app.add_solve(Solve {
                    time,
                    penalty: Penalty::None,
                    scramble,
                    timestamp: SystemTime::now(),
                });
                app.next_scramble();
            } else if matches!(code, KeyCode::Char(' ')) {
                handle_space(app, kind)?
            } else if !matches!(kind, KeyEventKind::Release)
                && matches!(app.timer_state, TimerState::Idle { .. })
            {
                handle_key(app, code)?
            }
        }
    }
    Ok(())
}
