use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Clear, Widget},
    Frame,
};

pub fn draw<B, W>(f: &mut Frame<B>, widget: W)
where
    B: Backend,
    W: Widget
{
    let size = f.size();
    let area = centered_rect(80, 7, size);

    f.render_widget(Clear, area);
    f.render_widget(widget, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
