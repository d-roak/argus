use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, Borders, List, ListItem, 
    },
    Frame,
};
use crate::global_state::State;

pub fn draw<B>(f: &mut Frame<B>, state: &mut State, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Min(8),
            ]
            .as_ref(),
        )
        .split(area);
    
    draw_transaction(f, state, chunks[0]);
}

fn draw_transaction<B>(f: &mut Frame<B>, state: &mut State, area: Rect)
where
    B: Backend,
{
    let tx_info: Vec<ListItem> = state
        .tx_info
        .items
        .iter() 
        .map(|i| ListItem::new(vec![
            Spans::from(vec![
                Span::raw(&i.0),
                Span::raw(" "),
                Span::styled(
                    &i.1,
                    Style::default().fg(Color::Yellow),
                ),
            ])
        ]))
        .collect();
    let tx_info = List::new(tx_info)
        .block(Block::default().borders(Borders::ALL).title("Transaction Information"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(tx_info, area, &mut state.tx_info.state);
}

