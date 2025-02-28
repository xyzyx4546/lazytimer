use std::time::Duration;

use ratatui::{prelude::*, widgets::*};

use crate::app::App;

pub struct AverageWidget {
    best: Option<Duration>,
    ao5: Option<Duration>,
    ao12: Option<Duration>,
}

impl AverageWidget {
    pub fn new(app: &App) -> AverageWidget {
        AverageWidget {
            best: app.current_session.best_time(),
            ao5: app.current_session.calculate_average(5),
            ao12: app.current_session.calculate_average(12),
        }
    }
}

impl Widget for AverageWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Averages")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let format_duration = |duration: Option<Duration>| match duration {
            Some(d) => format!("{:.2}", d.as_secs_f64()),
            None => "-".to_string(),
        };

        let text = vec![
            Line::from(vec![
                Span::raw("Best: "),
                Span::styled(
                    format_duration(self.best),
                    Style::default().fg(Color::Green),
                ),
            ]),
            Line::from(vec![
                Span::raw("ao5:  "),
                Span::styled(
                    format_duration(self.ao5),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Line::from(vec![
                Span::raw("ao12: "),
                Span::styled(format_duration(self.ao12), Style::default().fg(Color::Blue)),
            ]),
        ];

        Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}
