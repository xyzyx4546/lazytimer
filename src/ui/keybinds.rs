use ratatui::{prelude::*, widgets::*};

fn line<'a>(key: &'a str, value: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(format!("{:>8}", key), Style::default().fg(Color::Magenta)),
        Span::raw(format!("  {}", value)),
    ])
}

pub fn render(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Keybinds")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let text = vec![
        Line::raw(""),
        line("?", "Show keybinds"),
        line("q", "Quit"),
        line("Esc", "Close popup"),
        line("Enter", "Confirm"),
        line("Space", "Start/stop timer"),
        Line::raw(""),
        line("h / ←", "Previous puzzle type"),
        line("j / ↓", "Previous solve"),
        line("k / ↑", "Next solve"),
        line("l / →", "Next puzzle type"),
        line("g", "Go to first solve"),
        line("G", "Go to last solve"),
        Line::raw(""),
        line("i", "Show solve details"),
        line("+", "Toggle +2 penalty"),
        line("-", "Toggle DNF penalty"),
        line("d", "Delete selected solve"),
    ];

    let widget = Paragraph::new(text).block(block);

    frame.render_widget(widget, area);
}
