use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Graph")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let data: Vec<(f64, f64)> = app
        .selected_session()
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            s.effective_time()
                .map(|d| ((i + 1) as f64, d.as_secs_f64()))
        })
        .collect();

    let y_max = app
        .worst_time()
        .map_or(0.0, |d| (d.as_secs_f64() * 1.2).ceil());

    let widget = Chart::new(vec![
        Dataset::default()
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Magenta))
            .data(&data)
            .graph_type(GraphType::Line),
    ])
    .x_axis(Axis::default().bounds([1.0, app.selected_session().len() as f64]))
    .y_axis(Axis::default().bounds([0.0, y_max]).labels(vec![
        Span::raw("0.0"),
        Span::raw(format!("{:.1}", y_max / 3.0)),
        Span::raw(format!("{:.1}", y_max / 3.0 * 2.0)),
        Span::raw(format!("{:.1}", y_max)),
    ]))
    .block(block);

    frame.render_widget(widget, area);
}
