use crate::app::{App, Screen, TimerState};
use ratatui::{prelude::*, widgets::*, DefaultTerminal};
use std::io::Result;

mod widgets;
use widgets::*;

fn draw_timer(frame: &mut Frame, app: &App) {
    if !matches!(app.timer_state, TimerState::Idle { .. }) {
        frame.render_widget(TimerWidget::new(&app.timer_state), frame.area());
        return;
    }

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    frame.render_widget(ScrambleWidget::new(&app.current_scramble), layout[0]);
    frame.render_widget(TimerWidget::new(&app.timer_state), layout[1]);
}

fn draw_statistics(frame: &mut Frame, _app: &App) {
    let paragraph = Paragraph::new("WIP")
        .block(
            Block::default()
                .title("Statistics")
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, frame.area());
}

pub fn draw(app: &App, terminal: &mut DefaultTerminal) -> Result<()> {
    terminal.draw(|frame| match app.current_screen {
        Screen::Timer => draw_timer(frame, app),
        Screen::Statistics => draw_statistics(frame, app),
    })?;
    Ok(())
}
