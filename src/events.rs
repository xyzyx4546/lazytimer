use anyhow::{Context, Result};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind};
use std::time::{Duration, Instant, SystemTime};
use strum::IntoEnumIterator;

use crate::app::{App, DeletionTarget, PopupType, TimerState, INSPECTION_TIME};
use crate::sessions::{Penalty, PuzzleType, Solve};

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
            app.popup = None;
            return Ok(());
        }
        match popup_type {
            PopupType::ConfirmDelete { target } => {
                if matches!(code, KeyCode::Enter) {
                    match target {
                        DeletionTarget::Solve => {
                            let idx = app.selected_solve_idx;
                            app.selected_solve_idx = app.selected_solve_idx.saturating_sub(1);
                            app.selected_session_mut().solves.remove(idx);
                        }
                        DeletionTarget::Session => {
                            let idx = app.selected_session_idx;
                            app.selected_session_idx -= 1;
                            app.sessions.remove(idx);
                        }
                    }
                    app.popup = None;
                }
            }
            PopupType::CreateSession {
                name_buffer,
                selected_puzzle_type,
            } => match code {
                KeyCode::Esc => app.popup = None,
                KeyCode::Enter => {
                    if !name_buffer.trim().is_empty() {
                        let name = name_buffer.trim().to_string();
                        let puzzle_type = selected_puzzle_type.clone();
                        app.add_session(name, puzzle_type);
                        app.popup = None;
                    }
                }
                KeyCode::Tab => {
                    let puzzle_types: Vec<PuzzleType> = PuzzleType::iter().collect();
                    if let Some(current_index) =
                        puzzle_types.iter().position(|p| p == selected_puzzle_type)
                    {
                        let next_idx = (current_index + 1) % puzzle_types.len();
                        *selected_puzzle_type = puzzle_types[next_idx].clone();
                    }
                }
                _ => match code {
                    KeyCode::Char(c) => name_buffer.push(c),
                    KeyCode::Backspace => {
                        name_buffer.pop();
                    }
                    _ => {}
                },
            },
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
                app.popup = Some(PopupType::ConfirmDelete {
                    target: DeletionTarget::Solve,
                })
            }
            KeyCode::Char('D') => {
                app.popup = Some(PopupType::ConfirmDelete {
                    target: DeletionTarget::Session,
                })
            }
            KeyCode::Char('n') => {
                app.popup = Some(PopupType::CreateSession {
                    name_buffer: String::new(),
                    selected_puzzle_type: PuzzleType::ThreeByThree,
                });
            }
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
                if let Some(solve) = app.selected_solve_mut() {
                    solve.toggle_panalty(Penalty::PlusTwo);
                }
            }
            KeyCode::Char('-') => {
                if let Some(solve) = app.selected_solve_mut() {
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
