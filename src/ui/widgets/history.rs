use ratatui::{prelude::*, widgets::*};

use crate::app::{App, Penalty, Solve};

pub struct HistoryWidget {
    solves: Vec<Solve>,
}

impl HistoryWidget {
    pub fn new(app: &App) -> HistoryWidget {
        HistoryWidget {
            solves: app.current_session.solves.clone()
        }
    }
}

impl Widget for HistoryWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("History")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        // Create list items from solve times
        let text: Text = self.solves
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
                    Span::styled(
                        penalty,
                        Style::default().fg(Color::Red),
                    )
                ])
            })
            .collect::<Vec<Line>>()
            .into();

        // Render as paragraph
        Paragraph::new(text)
            .block(block)
            .render(area, buf);
    }
}
