use ratatui::{prelude::*, widgets::*};

use crate::app::App;

pub struct SessionWidget {
    name: String,
}

impl SessionWidget {
    pub fn new(app: &App) -> Self {
        Self {
            name: app.current_session.name.clone(),
        }
    }
}

impl Widget for SessionWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Session")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let inner = block.inner(area);
        block.render(area, buf);

        Paragraph::new(self.name)
            .centered()
            .render(inner, buf);
    }
}

