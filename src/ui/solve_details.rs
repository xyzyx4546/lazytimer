use crate::{app::App, sessions::Penalty, time_display::TimeDisplay};
use jiff::tz::TimeZone;
use ratatui::{prelude::*, widgets::*};
use tui_widgets::big_text::*;

fn line(key: &str, value: impl std::fmt::Display, color: Color) -> Line<'_> {
    Line::from(vec![
        Span::styled(
            format!("{:>10}  ", key),
            Style::default().fg(Color::Magenta),
        ),
        Span::styled(value.to_string(), Style::default().fg(color)),
    ])
}

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(format!("Solve #{}", app.selected_solve_idx + 1))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let solve = app.selected_solve().unwrap();

    let ao_str = |k: usize| -> String {
        app.ao(k)
            .get(app.selected_solve_idx)
            .and_then(|r| *r)
            .map_or("-".to_string(), |d| d.format(3))
    };

    let time_color = match solve.penalty {
        Penalty::None => Color::Green,
        Penalty::PlusTwo => Color::Yellow,
        Penalty::Dnf => Color::Red,
    };

    let big_text = BigText::builder()
        .lines(vec![solve.format(3).into()])
        .centered()
        .style(Style::default().fg(time_color).bold())
        .pixel_size(PixelSize::Sextant)
        .build();

    let timestamp_str = solve
        .timestamp
        .to_zoned(TimeZone::system())
        .strftime("%Y-%m-%d %H:%M");

    let text = vec![
        line("AO5", ao_str(5), Color::Blue),
        line("AO12", ao_str(12), Color::Cyan),
        line("Scramble", &solve.scramble, Color::White),
        line("Date", timestamp_str, Color::DarkGray),
    ];

    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(4),
        Constraint::Min(0),
    ])
    .split(block.inner(area));

    frame.render_widget(block, area);
    frame.render_widget(big_text, layout[1]);
    frame.render_widget(Paragraph::new(text), layout[2]);
}
