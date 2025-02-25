use crate::app::{App, Screen, TimerState};
use ratatui::{prelude::*, widgets::*, DefaultTerminal};
use std::io::Result;

mod widgets;
use widgets::*;

fn draw_timer(frame: &mut Frame, app: &App) {
    if !matches!(app.timer_state, TimerState::Idle { .. }) {
        frame.render_widget(TimerWidget::new(&app), frame.area());
        return;
    }

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(frame.area());

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(main_layout[0]);

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(main_layout[2]);

    frame.render_widget(
        SessionWidget::new(&app),
        left_layout[0],
    );
    frame.render_widget(
        HistoryWidget::new(&app),
        left_layout[1],
    );
    frame.render_widget(ScrambleWidget::new(&app), right_layout[0]);
    frame.render_widget(TimerWidget::new(&app), right_layout[1]);
}

fn draw_statistics(frame: &mut Frame, _app: &App) {
    let paragraph = Paragraph::new("WIP")
        .block(
            Block::default()
                .title("Statistics")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .centered();
    frame.render_widget(paragraph, frame.area());
}

pub fn draw(app: &App, terminal: &mut DefaultTerminal) -> Result<()> {
    terminal.draw(|frame| match app.current_screen {
        Screen::Timer => draw_timer(frame, app),
        Screen::Statistics => draw_statistics(frame, app),
    })?;
    Ok(())
}
