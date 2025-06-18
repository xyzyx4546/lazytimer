use ratatui::{prelude::*, widgets::*};
use strum::IntoEnumIterator;

use crate::sessions::PuzzleType;

pub struct Popup<'a> {
    name_buffer: &'a String,
    selected_puzzle_type: &'a PuzzleType,
}

impl<'a> Popup<'a> {
    pub fn new(name_buffer: &'a String, selected_puzzle_type: &'a PuzzleType) -> Self {
        Self {
            name_buffer,
            selected_puzzle_type,
        }
    }
}

impl<'a> Widget for Popup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Create New Session")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner = block.inner(area);
        block.render(area, buf);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(3),
            ])
            .split(inner);

        Paragraph::new(Span::raw(format!(" {}█", self.name_buffer)))
            .block(
                Block::default()
                    .title("Name")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .render(layout[0], buf);

        let spans: Vec<Span> = PuzzleType::iter()
            .map(|p| {
                if p == *self.selected_puzzle_type {
                    Span::styled(
                        format!(" {} ", p.to_string()),
                        Style::default()
                            .bg(Color::Blue)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    Span::raw(format!(" {} ", p.to_string()))
                }
            })
            .collect();

        Paragraph::new(Line::from(spans).centered())
            .block(
                Block::default()
                    .title("Puzzle")
                    .title_bottom(Line::from("Press <Tab> to cycle─").right_aligned())
                    .borders(Borders::all())
                    .border_type(BorderType::Rounded),
            )
            .render(layout[2], buf);
    }
}
