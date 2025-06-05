use ratatui::{prelude::*, widgets::*};

use crate::{app::App, sessions::Penalty};

pub struct History<'a> {
    app: &'a mut App,
}

impl<'a> History<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for History<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("History")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let text: Text = self.app
            .current_session()
            .solves
            .iter()
            .rev()
            .map(|solve| {
                let time_secs = solve.time.as_millis() as f64 / 1000.0;
                let penalty = match solve.penalty {
                    Penalty::None => "None",
                    Penalty::PlusTwo => "+2",
                    Penalty::Dnf => "DNF",
                };

                Line::from(vec![
                    Span::styled(
                        format!("{:>6.2}", time_secs),
                        Style::default().fg(Color::LightGreen),
                    ),
                    Span::raw(" "),
                    Span::styled(penalty, Style::default().fg(Color::Red)),
                ])
            })
            .collect::<Vec<Line>>()
            .into();

        Paragraph::new(text).block(block).render(area, buf);
    }
}
