use crate::app::{App, INSPECTION_TIME, TimerState};
use ratatui::{prelude::*, widgets::*};
use tui_widgets::big_text::*;

pub struct Timer<'a> {
    app: &'a App,
}

impl<'a> Timer<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for Timer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Timer")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let (text, style) = match self.app.timer_state {
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

        let inner = block.inner(area);
        block.render(area, buf);

        let big_text = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![text.as_str().into()])
            .style(style)
            .centered()
            .build();

        let vertical = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(4),
            Constraint::Min(1),
        ]);
        let [_, center, _] = vertical.areas(inner);
        big_text.render(center, buf);
    }
}
