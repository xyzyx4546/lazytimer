use crate::{app::App, sessions::PuzzleType};
use ratatui::{prelude::*, widgets::*};
use strum::IntoEnumIterator;

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Session")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let spans: Vec<Span> = PuzzleType::iter()
        .map(|p| {
            if p == app.selected_puzzle_type {
                Span::styled(
                    format!(" {} ", p.to_string()),
                    Style::default().bg(Color::Blue).fg(Color::Black),
                )
            } else {
                Span::raw(format!(" {} ", p.to_string()))
            }
        })
        .collect();

    let widget = Paragraph::new(Line::from(spans)).centered().block(block);

    frame.render_widget(widget, area);
}
