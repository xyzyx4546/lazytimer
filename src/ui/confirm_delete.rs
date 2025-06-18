use ratatui::{prelude::*, widgets::*};

use crate::app::{App, DeletionTarget};

pub struct Popup<'a> {
    app: &'a App,
    target: &'a DeletionTarget,
}

impl<'a> Popup<'a> {
    pub fn new(app: &'a App, target: &'a DeletionTarget) -> Self {
        Self { app, target }
    }
}

impl<'a> Widget for Popup<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::Red));

        let text = vec![Line::from(match self.target {
            DeletionTarget::Solve => format!(
                "Are you sure you want to delete Solve #{}?",
                self.app.selected_solve_idx + 1
            ),
            DeletionTarget::Session => format!(
                "Are you sure you want to delete Session \"{}\"?",
                &self.app.selected_session().name
            ),
        })];

        Paragraph::new(text)
            .alignment(Alignment::Center)
            .block(block)
            .render(area, buf);
    }
}
