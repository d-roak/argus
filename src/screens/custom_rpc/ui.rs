use crate::global::state::State;
use crate::components::modal;
use ratatui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Borders, Block, Paragraph},
    Frame,
};

pub fn draw<B>(f: &mut Frame<B>, state: &mut State)
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
            "Custom RPC",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL);

    let input = Paragraph::new(input)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true });

    modal::ui::draw(f, input);
}
