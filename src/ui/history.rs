use crate::{app::App, sessions::Penalty};
use ratatui::{prelude::*, widgets::*};

pub struct History<'a> {
    app: &'a App,
}

impl<'a> History<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for History<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("History")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner_area = block.inner(area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(inner_area);

        block.render(area, buf);

        let header = Line::from(vec![
            Span::styled("solve ", Style::default().fg(Color::White)),
            Span::raw("│    "),
            Span::styled("time     ", Style::default().fg(Color::Green)),
            Span::raw("│    "),
            Span::styled("ao5      ", Style::default().fg(Color::Blue)),
            Span::raw("│    "),
            Span::styled("ao12     ", Style::default().fg(Color::Cyan)),
        ]);
        Paragraph::new(header).render(chunks[0], buf);

        Paragraph::new(Line::from(
            "──────┼─────────────┼─────────────┼─────────────",
        ))
        .render(chunks[1], buf);

        let (ao5_times, ao12_times) = (self.app.ao(5), self.app.ao(12));
        let items: Vec<ListItem> = self
            .app
            .selected_session()
            .iter()
            .enumerate()
            .rev()
            .map(|(index, solve)| {
                let time = match solve.penalty {
                    Penalty::None => format!("{:.3}", solve.time.as_millis() as f64 / 1000.0),
                    Penalty::PlusTwo => {
                        format!("{:.3}+", solve.time.as_millis() as f64 / 1000.0 + 2.0)
                    }
                    Penalty::Dnf => "DNF".to_string(),
                };
                let ao5 =
                    ao5_times[index].map_or("-".to_string(), |d| format!("{:.3}", d.as_secs_f64()));
                let ao12 = ao12_times[index]
                    .map_or("-".to_string(), |d| format!("{:.3}", d.as_secs_f64()));

                ListItem::new(Line::from(vec![
                    Span::raw(format!("{:<6}", index + 1)),
                    Span::raw("│"),
                    Span::styled(
                        format!("{:^13}", time),
                        Style::default().fg(match solve.penalty {
                            Penalty::None => Color::Green,
                            Penalty::PlusTwo => Color::Yellow,
                            Penalty::Dnf => Color::Red,
                        }),
                    ),
                    Span::raw("│"),
                    Span::styled(format!("{:^13}", ao5), Style::default().fg(Color::Blue)),
                    Span::raw("│"),
                    Span::styled(format!("{:^13}", ao12), Style::default().fg(Color::Cyan)),
                ]))
            })
            .collect();

        let list = List::new(items).highlight_style(Style::default().bg(Color::DarkGray));
        let mut list_state = ListState::default();

        if !self.app.selected_session().is_empty() {
            list_state.select(Some(
                self.app.selected_session().len() - 1 - self.app.selected_solve_idx,
            ));
        }

        StatefulWidget::render(list, chunks[2], buf, &mut list_state);
    }
}
