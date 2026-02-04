use crate::app::{App, INSPECTION_TIME, TimerState};
use ratatui::{prelude::*, widgets::*};
use tui_widgets::big_text::*;

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Timer")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let (text, style) = match app.timer_state {
        TimerState::Idle { time } => (format!("{:.2}", time.as_secs_f64()), Style::default()),
        TimerState::PreInspection { time } => (
            format!("{:.2}", time.as_secs_f64()),
            Style::default().fg(Color::Yellow),
        ),
        TimerState::Inspection { start } => {
            let elapsed = start.elapsed().as_secs();
            let remaining = if elapsed >= INSPECTION_TIME {
                1
            } else {
                INSPECTION_TIME - elapsed
            };
            let style = if remaining <= 5 {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Green)
            };
            (format!("{remaining}"), style)
        }
        TimerState::PreRunning { start } => (
            format!("{}", INSPECTION_TIME - start.elapsed().as_secs()),
            Style::default().fg(Color::Yellow),
        ),
        TimerState::Running { start } => (
            format!("{:.1}", start.elapsed().as_secs_f64()),
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
