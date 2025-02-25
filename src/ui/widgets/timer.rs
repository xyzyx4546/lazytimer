use crate::app::{App, TimerState, INSPECTION_TIME};
use ratatui::{prelude::*, widgets::*};
use tui_widgets::big_text::*;

pub struct TimerWidget {
    text: String,
    style: Style,
}

impl TimerWidget {
    pub fn new(app: &App) -> TimerWidget {
        let (text, style) = match app.timer_state {
            TimerState::Idle { time } => (format!("{:.2}", time.as_secs_f64()), Style::default()),

            TimerState::PreInspection { time } => (
                format!("{:.2}", time.as_secs_f64()),
                Style::default().fg(Color::Yellow),
            ),

            TimerState::Inspection { start } => {
                let remaining = INSPECTION_TIME - start.elapsed().as_secs();
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
                format!("{:.2}", start.elapsed().as_secs_f64()),
                Style::default().fg(Color::Green),
            ),
        };

        TimerWidget { text, style }
    }
}

impl Widget for TimerWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Timer")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner = block.inner(area);
        block.render(area, buf);

        let big_text = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![self.text.as_str().into()])
            .style(self.style)
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
