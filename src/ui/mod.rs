use crate::app::{App, PopupType, TimerState};
use anyhow::Result;
use ratatui::{DefaultTerminal, prelude::*, widgets::*};

mod confirm_delete;
mod graph;
mod history;
mod keybinds;
mod scramble;
mod session;
mod solve_details;
mod stats;
mod timer;

pub fn draw(app: &App, terminal: &mut DefaultTerminal) -> Result<()> {
    terminal.draw(|frame| {
        let main_layout = Layout::horizontal([
            Constraint::Length(50),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(frame.area());

        let left_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Length(12),
            Constraint::Min(0),
        ])
        .split(main_layout[0]);

        let scramble_height = {
            let width = main_layout[2].width.saturating_sub(4);
            if width == 0 {
                3
            } else {
                (app.current_scramble.len() as u16 / width).saturating_add(3)
            }
        };

        let right_layout =
            Layout::vertical([Constraint::Length(scramble_height), Constraint::Min(0)])
                .split(main_layout[2]);

        if !app.sessions.is_empty() {
            match app.timer_state {
                TimerState::Idle { .. } => {
                    session::render(app, frame, left_layout[0]);
                    stats::render(app, frame, left_layout[1]);
                    graph::render(app, frame, left_layout[2]);
                    history::render(app, frame, left_layout[3]);
                    scramble::render(app, frame, right_layout[0]);
                    timer::render(app, frame, right_layout[1]);
                }
                _ => {
                    timer::render(app, frame, frame.area());
                }
            }
        }

        if let Some(popup_type) = &app.popup {
            let (height, width) = match popup_type {
                PopupType::Keybinds => (21, 50),
                PopupType::ConfirmDelete => (3, 70),
                PopupType::SolveDetails => (12, 80),
            };

            let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length((frame.area().height.saturating_sub(height)) / 2),
                    Constraint::Length(height),
                    Constraint::Length((frame.area().height.saturating_sub(height)) / 2),
                ])
                .split(frame.area())[1];

            let area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length((frame.area().width.saturating_sub(width)) / 2),
                    Constraint::Length(width),
                    Constraint::Length((frame.area().width.saturating_sub(width)) / 2),
                ])
                .split(vertical)[1];

            frame.render_widget(Clear, area);

            match popup_type {
                PopupType::Keybinds => keybinds::render(frame, area),
                PopupType::ConfirmDelete => confirm_delete::render(app, frame, area),
                PopupType::SolveDetails => solve_details::render(app, frame, area),
            }
        }
    })?;
    Ok(())
}
