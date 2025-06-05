use std::time::Duration;

use ratatui::{prelude::*, widgets::*};

use crate::app::App;

pub struct Stats<'a> {
    app: &'a mut App,
}

impl<'a> Stats<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for Stats<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Stats")
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
                    format_duration(self.app.selected_session().best_time()),
                    Style::default().fg(Color::Green),
                ),
            ]),
            Line::from(vec![
                Span::raw("ao5:  "),
                Span::styled(
                    format_duration(self.app.selected_session().ao(5).last().cloned().flatten()),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Line::from(vec![
                Span::raw("ao12: "),
                Span::styled(
                    format_duration(self.app.selected_session().ao(12).last().cloned().flatten()),
                    Style::default().fg(Color::Blue),
                ),
            ]),
        ];

        Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}
