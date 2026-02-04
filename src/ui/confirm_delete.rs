use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Red));

    let text = vec![Line::from(format!(
        "Are you sure you want to delete Solve #{}?",
        app.selected_solve_idx + 1
    ))];

    let widget = Paragraph::new(text).centered().block(block);

    frame.render_widget(widget, area);
}
