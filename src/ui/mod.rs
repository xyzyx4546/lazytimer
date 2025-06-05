use anyhow::Result;
use ratatui::{prelude::*, DefaultTerminal};

use crate::app::{App, TimerState};

mod graph;
mod history;
mod scramble;
mod session;
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
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(main_layout[2]);

        if matches!(app.timer_state, TimerState::Idle { .. }) {
            frame.render_widget(session::Session::new(app), left_layout[0]);
            frame.render_widget(stats::Stats::new(app), left_layout[1]);
            frame.render_widget(graph::Graph::new(app), left_layout[2]);
            frame.render_widget(history::History::new(app), left_layout[3]);
            frame.render_widget(scramble::Scramble::new(app), right_layout[0]);
            frame.render_widget(timer::Timer::new(app), right_layout[1]);
        } else {
            frame.render_widget(timer::Timer::new(app), frame.area());
        }
    })?;
    Ok(())
}
