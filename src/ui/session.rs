use crate::{app::App, sessions::PuzzleType};
use ratatui::{prelude::*, widgets::*};
use strum::IntoEnumIterator;

pub struct Session<'a> {
    app: &'a App,
}

impl<'a> Session<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for Session<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Session")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner = block.inner(area);
        block.render(area, buf);

        let spans: Vec<Span> = PuzzleType::iter()
            .map(|p| {
                if p == self.app.selected_puzzle_type {
                    Span::styled(
                        format!(" {} ", p.to_string()),
                        Style::default().bg(Color::Blue).fg(Color::Black),
                    )
                } else {
                    Span::raw(format!(" {} ", p.to_string()))
                }
            })
            .collect();

        Paragraph::new(Line::from(spans))
            .centered()
            .render(inner, buf);
    }
}
