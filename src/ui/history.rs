use crate::{app::App, sessions::Penalty};
use ratatui::{prelude::*, widgets::*};

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("History")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let header = Line::from(vec![
        Span::styled("solve ", Style::default().fg(Color::White)),
        Span::raw("│    "),
        Span::styled("time     ", Style::default().fg(Color::Green)),
        Span::raw("│    "),
        Span::styled("ao5      ", Style::default().fg(Color::Blue)),
        Span::raw("│    "),
        Span::styled("ao12     ", Style::default().fg(Color::Cyan)),
    ]);

    let separator = "──────┼─────────────┼─────────────┼─────────────";

    let items: Vec<ListItem> = app
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
                app.ao(5)[index].map_or("-".to_string(), |d| format!("{:.3}", d.as_secs_f64()));
            let ao12 =
                app.ao(12)[index].map_or("-".to_string(), |d| format!("{:.3}", d.as_secs_f64()));

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

    if !app.selected_session().is_empty() {
        list_state.select(Some(
            app.selected_session().len() - 1 - app.selected_solve_idx,
        ));
    }

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(block.inner(area));

    frame.render_widget(block, area);
    frame.render_widget(header, layout[0]);
    frame.render_widget(separator, layout[1]);
    frame.render_stateful_widget(list, layout[2], &mut list_state);
}
