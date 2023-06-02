use crate::{app::App, global_state::State};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Borders, Block, Paragraph},
    Frame,
};

pub fn draw<B>(f: &mut Frame<B>, app: &mut App, state: &mut State, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);

    draw_search_box(f, app, state, chunks[0]);
}

fn draw_search_box<B>(f: &mut Frame<B>, app: &mut App, state: &mut State, area: Rect)
where
    B: Backend,
{
    let input = vec![
        Spans::from(vec![
            Span::styled(
                &state.input_buffer,
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let block = Block::default()
        .title(Span::styled(
            "Search (press I to insert mode, press Esc to normal mode)",
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let input = Paragraph::new(input)
        .style(Style::default().fg(Color::White))
        .block(block)
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(input, area);
}
