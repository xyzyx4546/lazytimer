use anyhow::Result;
use ratatui::{prelude::*, widgets::*, DefaultTerminal};

use crate::app::{App, PopupType, TimerState};

mod confirm_delete;
mod graph;
mod history;
mod keybinds;
mod scramble;
mod session;
mod solve_details;
mod stats;
mod timer;

pub fn draw(app: &mut App, terminal: &mut DefaultTerminal) -> Result<()> {
    terminal.draw(|frame| {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(50),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(frame.area());

        let left_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Length(12),
                Constraint::Min(0),
            ])
            .split(main_layout[0]);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(main_layout[2]);

        if matches!(app.timer_state, TimerState::Idle { .. }) {
            frame.render_widget(session::Session::new(app), left_layout[0]);
            frame.render_widget(stats::Stats::new(app), left_layout[1]);
            frame.render_widget(graph::Graph::new(app), left_layout[2]);
            frame.render_widget(history::History::new(app), left_layout[3]);
            frame.render_widget(scramble::Scramble::new(app), right_layout[0]);
            frame.render_widget(timer::Timer::new(app), right_layout[1]);

            fn render_popup(popup: impl Widget, frame: &mut Frame, height: u16) {
                let area = frame.area();
                let popup_area = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length((area.height.saturating_sub(height)) / 2),
                        Constraint::Length(height),
                        Constraint::Length((area.height.saturating_sub(height)) / 2),
                    ])
                    .split(area);

                let popup_area = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length((area.width.saturating_sub(80)) / 2),
                        Constraint::Length(80),
                        Constraint::Length((area.width.saturating_sub(80)) / 2),
                    ])
                    .split(popup_area[1])[1];

                frame.render_widget(Clear, popup_area);
                frame.render_widget(popup, popup_area);
            }

            if let Some(popup_type) = &app.popup {
                match popup_type {
                    PopupType::Keybinds => {
                        render_popup(keybinds::Popup::new(), frame, 15);
                    }
                    PopupType::ConfirmDelete => {
                        render_popup(confirm_delete::Popup::new(app), frame, 3);
                    }
                    PopupType::SolveDetails => {
                        render_popup(solve_details::Popup::new(app), frame, 12);
                    }
                };
            }
        } else {
            frame.render_widget(timer::Timer::new(app), frame.area());
        }
    })?;
    Ok(())
}
