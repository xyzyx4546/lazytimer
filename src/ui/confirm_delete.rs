use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub struct Popup<'a> {
    app: &'a App,
}

impl<'a> Popup<'a> {
    pub fn new(app: &'a App) -> Self {
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
