use crate::{
    app::{App, INSPECTION_TIME, PopupType, TimerState},
    sessions::{Penalty, Solve},
};
use anyhow::{Context, Result};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, poll, read};
use std::time::{Duration, Instant, SystemTime};

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
    if let Some(popup_type) = &mut app.popup {
        if matches!(code, KeyCode::Esc) {
            if app.sessions.is_empty() {
                app.exiting = true;
            } else {
                app.popup = None;
            }
            return Ok(());
        }
        match popup_type {
            PopupType::ConfirmDelete => {
                if matches!(code, KeyCode::Enter) {
                    let idx = app.selected_solve_idx;
                    app.selected_solve_idx = app.selected_solve_idx.saturating_sub(1);
                    app.selected_session_mut().remove(idx);
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
            KeyCode::Char('d') => {
                if app.selected_solve().is_some() {
                    app.popup = Some(PopupType::ConfirmDelete)
                }
            }
            KeyCode::Char('i') => {
                if app.selected_solve().is_some() {
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
                if let Some(solve) = app.selected_solve_mut() {
                    solve.toggle_penalty(Penalty::PlusTwo);
                }
            }
            KeyCode::Char('-') => {
                if let Some(solve) = app.selected_solve_mut() {
                    solve.toggle_penalty(Penalty::Dnf);
                }
            }
            _ => {}
        };
    }
    Ok(())
}

pub fn handle(app: &mut App) -> Result<()> {
    if let TimerState::Inspection { start } = app.timer_state {
        if start.elapsed().as_secs() >= INSPECTION_TIME {
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
                app.add_solve(Solve {
                    time,
                    penalty: Penalty::None,
                    scramble: app.current_scramble.clone(),
                    timestamp: SystemTime::now(),
                });
                app.next_scramble();
            } else if matches!(code, KeyCode::Char(' ')) && matches!(app.popup, None) {
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
