use ratatui::{prelude::*, widgets::*};

use crate::app::App;

pub struct Scramble<'a> {
    app: &'a mut App,
}

impl<'a> Scramble<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for Scramble<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Scramble")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner = block.inner(area);
        block.render(area, buf);

        Paragraph::new(self.app.current_scramble.to_string())
            .centered()
            .style(Style::new().fg(Color::Magenta))
            .render(inner, buf);
    }
}
