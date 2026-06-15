use crate::{
    app::{App, PopupType, TimerState},
    sessions::{Penalty, Solve},
};
use anyhow::{Context, Result};
use crokey::KeyCombination;
use crossterm::event::{Event, KeyEventKind, poll, read};
use jiff::Timestamp;
use std::time::{Duration, Instant};

pub fn handle_timer_key(app: &mut App, kind: KeyEventKind) -> Result<()> {
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
                TimerState::PreInspection { .. } => {
                    if app.config.timer.inspection_time > 0 {
                        TimerState::Inspection {
                            start: Instant::now(),
                        }
                    } else {
                        TimerState::Running {
                            start: Instant::now(),
                        }
                    }
                }
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

pub fn handle_key(app: &mut App, key: KeyCombination) -> Result<()> {
    let binds = &app.config.keybinds;

    if let Some(popup_type) = &mut app.popup {
        if key == binds.cancel {
            app.popup = None;
            return Ok(());
        }

        #[allow(clippy::single_match)]
        match popup_type {
            PopupType::ConfirmDelete if key == binds.confirm => {
                app.delete_solve();
                app.popup = None;
            }
            _ => {}
        }
    } else {
        match key {
            c if c == binds.previous_puzzle => app.switch_session(-1),
            c if c == binds.previous_solve => app.switch_solve(-1),
            c if c == binds.next_puzzle => app.switch_session(1),
            c if c == binds.next_solve => app.switch_solve(1),
            c if c == binds.first_solve => app.reset_selected_solve(),
            c if c == binds.last_solve => app.selected_solve_idx = 0,

            c if c == binds.quit => {
                app.save_sessions().context("Failed to save sessions")?;
                app.exiting = true;
            }
            c if c == binds.show_keybinds => {
                app.popup = Some(PopupType::Keybinds);
            }
            c if c == binds.toggle_ghost_mode => {
                app.ghost_mode = !app.ghost_mode;
            }

            c if c == binds.delete_solve && app.selected_solve().is_some() => {
                app.popup = Some(PopupType::ConfirmDelete);
            }
            c if c == binds.solve_details && app.selected_solve().is_some() => {
                app.popup = Some(PopupType::SolveDetails);
            }
            c if c == binds.toggle_plus_two => {
                if let Some(solve) = app.selected_solve_mut() {
                    solve.toggle_penalty(Penalty::PlusTwo);
                    app.compute_averages();
                }
            }
            c if c == binds.toggle_dnf => {
                if let Some(solve) = app.selected_solve_mut() {
                    solve.toggle_penalty(Penalty::Dnf);
                    app.compute_averages();
                }
            }
            _ => {}
        };
    }
    Ok(())
}

pub fn handle(app: &mut App) -> Result<()> {
    if let TimerState::Inspection { start } = app.timer_state
        && start.elapsed().as_secs() >= app.config.timer.inspection_time
    {
        app.timer_state = TimerState::Running {
            start: Instant::now(),
        };
    }

    if matches!(app.timer_state, TimerState::Idle { .. }) || poll(Duration::from_millis(100))? {
        let Event::Key(key_event) = read()? else {
            return Ok(());
        };
        let key = KeyCombination::from(key_event);
        let kind = key_event.kind;

        if !app.ghost_mode
            && let TimerState::Running { start } = app.timer_state
        {
            let time = start.elapsed();
            app.timer_state = TimerState::Idle { time };
            app.add_solve(Solve {
                time,
                penalty: Penalty::None,
                scramble: app.current_scramble.clone(),
                timestamp: Timestamp::now(),
            });
            app.next_scramble();
            return Ok(());
        }

        if key == app.config.keybinds.start_timer && app.popup.is_none() {
            handle_timer_key(app, kind)?;
        } else if kind != KeyEventKind::Release {
            handle_key(app, key)?;
        }
    }
    Ok(())
}
