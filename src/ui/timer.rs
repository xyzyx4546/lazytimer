use crate::{
    app::{App, INSPECTION_TIME, TimerState},
    time_display::TimeDisplay,
};
use ratatui::{prelude::*, widgets::*};
use std::cmp::max;
use tui_widgets::big_text::*;

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Timer")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let (text, style) = match app.timer_state {
        TimerState::Idle { time } => (time.format(2), Style::default()),
        TimerState::PreInspection { time } => (time.format(2), Style::default().fg(Color::Yellow)),
        TimerState::Inspection { start } => {
            let remaining = max(INSPECTION_TIME - start.elapsed().as_secs(), 1);
            let style = if remaining <= 5 {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Green)
            };
            (remaining.to_string(), style)
        }
        TimerState::PreRunning { start } => (
            start.elapsed().format(0),
            Style::default().fg(Color::Yellow),
        ),
        TimerState::Running { start } => {
            (start.elapsed().format(1), Style::default().fg(Color::Green))
        }
    };

    let big_text = BigText::builder()
        .lines(vec![text.into()])
        .centered()
        .style(style)
        .pixel_size(PixelSize::HalfHeight)
        .build();

    let [_, center, _] = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(4),
        Constraint::Min(1),
    ])
    .areas(block.inner(area));

    frame.render_widget(block, area);
    frame.render_widget(big_text, center);
}
