use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Scramble")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::reset())
        .padding(Padding::horizontal(1));

    let widget = Paragraph::new(app.current_scramble.to_string())
        .centered()
        .style(Style::new().fg(Color::Magenta))
        .wrap(Wrap::default())
        .block(block);

    frame.render_widget(widget, area);
}
