use crate::{app::App, sessions::Penalty, time_display::TimeDisplay};
use ratatui::{prelude::*, widgets::*};

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("History")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let header = Line::from(vec![
        Span::styled("solve ", Color::White),
        Span::raw("│    "),
        Span::styled("time     ", Color::Green),
        Span::raw("│    "),
        Span::styled("ao5      ", Color::Blue),
        Span::raw("│    "),
        Span::styled("ao12     ", Color::Cyan),
    ]);

    let separator = "──────┼─────────────┼─────────────┼─────────────";

    let items: Vec<ListItem> = app
        .selected_session()
        .iter()
        .enumerate()
        .rev()
        .map(|(index, solve)| {
            let ao5 = app.ao5[index].map_or("-".to_string(), |d| d.format(3));
            let ao12 = app.ao12[index].map_or("-".to_string(), |d| d.format(3));

            ListItem::new(Line::from(vec![
                Span::raw(format!("{:<6}", index + 1)),
                Span::raw("│"),
                Span::styled(
                    format!("{:^13}", solve.format(3)),
                    match solve.penalty {
                        Penalty::None => Color::Green,
                        Penalty::PlusTwo => Color::Yellow,
                        Penalty::Dnf => Color::Red,
                    },
                ),
                Span::raw("│"),
                Span::styled(format!("{:^13}", ao5), Color::Blue),
                Span::raw("│"),
                Span::styled(format!("{:^13}", ao12), Color::Cyan),
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
