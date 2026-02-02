use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub struct Graph<'a> {
    app: &'a App,
}

impl<'a> Graph<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl<'a> Widget for Graph<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let data: Vec<(f64, f64)> = self
            .app
            .selected_session()
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                s.effective_time()
                    .map(|d| ((i + 1) as f64, d.as_secs_f64()))
            })
            .collect();

        let y_max = self
            .app
            .worst_time()
            .map_or(0.0, |d| (d.as_secs_f64() * 1.2).ceil());

        Chart::new(vec![
            Dataset::default()
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Magenta))
                .data(&data)
                .graph_type(GraphType::Line),
        ])
        .block(
            Block::default()
                .title("Graph")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .x_axis(Axis::default().bounds([1.0, self.app.selected_session().len() as f64]))
        .y_axis(Axis::default().bounds([0.0, y_max]).labels(vec![
            Span::raw("0.0"),
            Span::raw(format!("{:.1}", y_max / 3.0)),
            Span::raw(format!("{:.1}", y_max / 3.0 * 2.0)),
            Span::raw(format!("{:.1}", y_max)),
        ]))
        .render(area, buf);
    }
}
