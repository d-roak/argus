use crate::global_state::State;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Line},
    widgets::{
        Block, Borders, Clear, List, ListItem, 
    },
    Frame,
};

pub fn draw<B>(f: &mut Frame<B>, state: &mut State)
where
    B: Backend,
{
    let size = f.size();
    let area = centered_rect(23, 25, size);
    f.render_widget(Clear, area);
    draw_list_rpcs(f, state, area);
}

fn draw_list_rpcs<B>(f: &mut Frame<B>, state: &mut State, area: Rect)
where
    B: Backend,
{
    let list: Vec<ListItem> = state
        .rpc_list
        .items
        .iter() 
        .map(|i| ListItem::new(vec![Line::from(Span::raw(&i.1))]))
        .collect();
    let list = List::new(list)
        .block(Block::default().borders(Borders::ALL).title("Available RPCs"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, area, &mut state.rpc_list.state);
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
