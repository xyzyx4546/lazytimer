use ratatui::{prelude::*, widgets::*};

use crate::app::App;

pub struct Session<'a> {
    app: &'a mut App,
}

impl<'a> Session<'a> {
    pub fn new(app: &'a mut App) -> Self {
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

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(inner);

        let left_area = layout[0];
        let middle_area = layout[1];
        let right_area = layout[2];

        let has_previous = self.app.selected_session_idx > 0;
        let has_next = self.app.selected_session_idx < self.app.sessions.len() - 1;

        let left_text = if has_previous { "  <" } else { "" };
        Paragraph::new(left_text).render(left_area, buf);

        Paragraph::new(format!(
            "{} ({})",
            self.app.selected_session().name.clone(),
            self.app.selected_session().puzzle_type.to_string()
        ))
        .centered()
        .render(middle_area, buf);

        let right_text = if has_next { ">  " } else { "" };
        Paragraph::new(right_text).render(right_area, buf);
    }
}
