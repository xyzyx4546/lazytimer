use ratatui::{prelude::*, widgets::*};
use std::time::Duration;
use tui_widgets::big_text::*;

use crate::{
    app::{App, TimerState, INSPECTION_TIME},
    sessions::Penalty,
};

fn render_session(app: &mut App, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title("Session")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let inner = block.inner(area);
    block.render(area, buf);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(inner);

    let left_area = layout[0];
    let middle_area = layout[1];
    let right_area = layout[2];

    let has_previous = app.current_session_idx > 0;
    let has_next = app.current_session_idx < app.sessions.len() - 1;

    let left_text = if has_previous { " <" } else { "" };
    Paragraph::new(left_text).render(left_area, buf);

    Paragraph::new(app.current_session().name.clone())
        .centered()
        .render(middle_area, buf);

    let right_text = if has_next { "> " } else { "" };
    Paragraph::new(right_text).render(right_area, buf);
}

fn render_averages(app: &mut App, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title("Averages")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let format_duration = |duration: Option<Duration>| match duration {
        Some(d) => format!("{:.2}", d.as_secs_f64()),
        None => "-".to_string(),
    };

    let text = vec![
        Line::from(vec![
            Span::raw("Best: "),
            Span::styled(
                format_duration(app.current_session().best_time()),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("ao5:  "),
            Span::styled(
                format_duration(app.current_session().calculate_average(5)),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("ao12: "),
            Span::styled(
                format_duration(app.current_session().calculate_average(12)),
                Style::default().fg(Color::Blue),
            ),
        ]),
    ];

    Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Left)
        .render(area, buf);
}

fn render_history(app: &mut App, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title("History")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let text: Text = app
        .current_session()
        .solves
        .iter()
        .rev()
        .map(|solve| {
            let time_secs = solve.time.as_millis() as f64 / 1000.0;
            let penalty = match solve.penalty {
                Penalty::None => "None",
                Penalty::PlusTwo => "+2",
                Penalty::Dnf => "DNF",
            };

            Line::from(vec![
                Span::styled(
                    format!("{:>6.2}", time_secs),
                    Style::default().fg(Color::LightGreen),
                ),
                Span::raw(" "),
                Span::styled(penalty, Style::default().fg(Color::Red)),
            ])
        })
        .collect::<Vec<Line>>()
        .into();

    Paragraph::new(text).block(block).render(area, buf);
}

fn render_scramble(app: &App, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title("Scramble")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let inner = block.inner(area);
    block.render(area, buf);

    Paragraph::new(app.current_scramble.to_string())
        .centered()
        .style(Style::new().fg(Color::Magenta))
        .render(inner, buf);
}

fn render_timer(app: &App, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title("Timer")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let (text, style) = match &app.timer_state {
        TimerState::Idle { time } => (format!("{:.2}", time.as_secs_f64()), Style::default()),
        TimerState::PreInspection { time } => (
            format!("{:.2}", time.as_secs_f64()),
            Style::default().fg(Color::Yellow),
        ),
        TimerState::Inspection { start } => {
            let elapsed = start.elapsed().as_secs();
            let remaining = if elapsed >= INSPECTION_TIME {
                1
            } else {
                INSPECTION_TIME - elapsed
            };
            let style = if remaining <= 5 {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Green)
            };
            (format!("{remaining}"), style)
        }
        TimerState::PreRunning { start } => (
            format!("{}", INSPECTION_TIME - start.elapsed().as_secs()),
            Style::default().fg(Color::Yellow),
        ),
        TimerState::Running { start } => (
            format!("{:.1}", start.elapsed().as_secs_f64()),
            Style::default().fg(Color::Green),
        ),
    };

    let inner = block.inner(area);
    block.render(area, buf);

    let big_text = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .lines(vec![text.as_str().into()])
        .style(style)
        .centered()
        .build();

    let vertical = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(4),
        Constraint::Min(1),
    ]);
    let [_, center, _] = vertical.areas(inner);
    big_text.render(center, buf);
}

pub struct TimerScreen<'a> {
    app: &'a mut App,
}

impl<'a> TimerScreen<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for TimerScreen<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !matches!(self.app.timer_state, TimerState::Idle { .. }) {
            render_timer(self.app, area, buf);
            return;
        }

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(20),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(area);

        let left_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(0),
            ])
            .split(main_layout[0]);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(main_layout[2]);

        render_session(self.app, left_layout[0], buf);
        render_averages(self.app, left_layout[1], buf);
        render_history(self.app, left_layout[2], buf);
        render_scramble(self.app, right_layout[0], buf);
        render_timer(self.app, right_layout[1], buf);
    }
}
