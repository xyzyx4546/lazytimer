use ratatui::{prelude::*, widgets::*};

use crate::app::App;

enum ChartType {
    Individual,
    Ao5,
    Ao12,
}

fn render_chart(app: &mut App, area: Rect, buf: &mut Buffer, chart_type: ChartType) {
    let session = app.current_session();

    let (title, color) = match chart_type {
        ChartType::Individual => ("Individual", Color::Green),
        ChartType::Ao5 => ("Ao5", Color::Yellow),
        ChartType::Ao12 => ("Ao12", Color::Blue),
    };

    let times = match chart_type {
        ChartType::Individual => session.solves.iter().map(|s| s.effective_time()).collect(),
        ChartType::Ao5 => session.ao(5),
        ChartType::Ao12 => session.ao(12),
    };

    let data: Vec<(f64, f64)> = times
        .iter()
        .enumerate()
        .filter_map(|(index, opt_duration)| {
            opt_duration.map(|duration| ((index + 1) as f64, duration.as_secs_f64()))
        })
        .collect();

    let y_max = session
        .worst_time()
        .map_or(0.0, |d| (d.as_secs_f64() * 1.2).ceil());

    Chart::new(vec![Dataset::default()
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(color))
        .data(&data)
        .graph_type(GraphType::Line)])
    .block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .x_axis(Axis::default().bounds([1.0, session.solves.len() as f64]))
    .y_axis(
        Axis::default()
            .bounds([0.0, y_max])
            .labels(vec![
                Span::raw("0.0"),
                Span::raw(format!("{:.1}", y_max / 2.0)),
                Span::raw(format!("{:.1}", y_max)),
            ]),
    )
    .render(area, buf);
}

fn render_list(_app: &App, area: Rect, buf: &mut Buffer) {
    let paragraph = Paragraph::new("WIP")
        .block(
            Block::default()
                .title("Statistics")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .centered();
    paragraph.render(area, buf);
}

pub struct StatisticsScreen<'a> {
    app: &'a mut App,
}

impl<'a> StatisticsScreen<'a> {
    pub fn new(app: &'a mut App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for StatisticsScreen<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Fill(1)])
            .split(area);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .split(main_layout[1]);

        render_list(self.app, main_layout[0], buf);
        render_chart(self.app, right_layout[0], buf, ChartType::Individual);
        render_chart(self.app, right_layout[1], buf, ChartType::Ao5);
        render_chart(self.app, right_layout[2], buf, ChartType::Ao12);
    }
}
