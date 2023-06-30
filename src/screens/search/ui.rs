use crate::global::state::State;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Borders, Block, Clear, Paragraph},
    Frame,
};

pub fn draw<B>(f: &mut Frame<B>, state: &mut State)
where
    B: Backend,
{
    let size = f.size();
    let area = centered_rect(80, 7, size);
    f.render_widget(Clear, area);
    draw_search_box(f, state, area);
}

fn draw_search_box<B>(f: &mut Frame<B>, state: &mut State, area: Rect)
where
    B: Backend,
{
    let input = vec![
        Line::from(vec![
            Span::styled(
                &state.input_buffer,
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let block = Block::default()
        .title(Span::styled(
            "Search",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL);

    let input = Paragraph::new(input)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(input, area);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
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
