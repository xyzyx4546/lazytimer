use crate::{
    app::{App, TimerState},
    time_display::TimeDisplay,
};
use ratatui::{prelude::*, widgets::*};
use std::time::Duration;
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
            let remaining = Duration::from_secs(app.config.timer.inspection_time)
                .saturating_sub(start.elapsed())
                .max(Duration::from_secs(1));

            let color = if remaining.as_secs() <= 5 {
                Color::Red
            } else {
                Color::Green
            };
            (remaining.format(0), Style::default().fg(color))
        }
        TimerState::PreRunning { start } => {
            let remaining = Duration::from_secs(app.config.timer.inspection_time)
                .saturating_sub(start.elapsed())
                .max(Duration::from_secs(1));

            (remaining.format(0), Style::default().fg(Color::Yellow))
        }
        TimerState::Running { start } => (
            if app.config.timer.hide_timer_while_solving {
                "SOLVE".to_string()
            } else {
                start.elapsed().format(1)
            },
            Style::default().fg(Color::Green),
        ),
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
