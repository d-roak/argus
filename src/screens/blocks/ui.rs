use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{
        Axis, Block, Borders, Chart, Dataset, List, ListItem, 
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
    
    draw_blocks_list(f, state, chunks[0]);
}

fn draw_blocks_list<B>(f: &mut Frame<B>, state: &mut State, area: Rect)
where
    B: Backend,
{
    let constraints = vec![Constraint::Percentage(20), Constraint::Percentage(80)];
    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);

    let blocks: Vec<ListItem> = state
        .blocks
        .items
        .iter() 
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(i))]))
        .collect();
    let blocks = List::new(blocks)
        .block(Block::default().borders(Borders::ALL).title("Last Blocks"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(blocks, chunks[0], &mut state.blocks.state);

    let block_info: Vec<ListItem> = state
        .block_info
        .items
        .iter() 
        .map(|i| ListItem::new(vec![
            Spans::from(vec![
                Span::raw(&i.0),
                // tab spacer
                Span::raw(" "),
                Span::styled(
                    &i.1,
                    Style::default().fg(Color::Yellow),
                ),
            ])
        ]))
        .collect();
    let block_info = List::new(block_info)
        .block(Block::default().borders(Borders::ALL).title("Block Information"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(block_info, chunks[1], &mut state.block_info.state);
}

