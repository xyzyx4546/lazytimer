use ratatui::{prelude::*, widgets::*};

use crate::app::App;

pub struct Popup<'a> {
    app: &'a mut App,
}

impl<'a> Popup<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for Popup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Red));

        let text = vec![Line::from(format!(
            "Are you sure you want to delete Solve #{}?",
            self.app.selected_solve_idx + 1
        ))];

        Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(block)
            .render(area, buf);
    }
}
