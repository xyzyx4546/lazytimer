use crokey::KeyCombination;
use ratatui::{prelude::*, widgets::*};
use crate::app::App;

fn line(key: KeyCombination, value: &'_ str) -> Line<'_> {
    Line::from(vec![
        Span::styled(format!("{:>10}", key.to_string()), Style::default().fg(Color::Magenta)),
        Span::raw(format!("   {}", value)),
    ])
}

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Keybinds")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let binds = &app.config.keybinds;
    let text = vec![
        Line::raw(""),
        Line::styled("             Navigation", Color::Blue),
        line(binds.previous_puzzle, "Previous puzzle type"),
        line(binds.previous_solve, "Previous solve"),
        line(binds.next_solve, "Next solve"),
        line(binds.next_puzzle, "Next puzzle type"),
        line(binds.first_solve, "Go to first solve"),
        line(binds.last_solve, "Go to last solve"),
        Line::raw(""),
        Line::styled("             Global Actions", Color::Blue),
        line(binds.show_keybinds, "Show keybinds"),
        line(binds.quit, "Quit"),
        line(binds.cancel, "Cancel"),
        line(binds.confirm, "Confirm"),
        line(binds.start_timer, "Start timer"),
        line(binds.toggle_ghost_mode, "Toggle ghost mode"),
        Line::raw(""),
        Line::styled("             Solve Actions", Color::Blue),
        line(binds.solve_details, "Show solve details"),
        line(binds.toggle_plus_two, "Toggle +2 penalty"),
        line(binds.toggle_dnf, "Toggle DNF penalty"),
        line(binds.delete_solve, "Delete selected solve"),
    ];

    let widget = Paragraph::new(text).block(block);

    frame.render_widget(widget, area);
}
