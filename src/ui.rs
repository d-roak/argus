use crate::global_state::State;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{
        Borders, Block, Tabs,
    },
    Frame,
};
use crate::screens;

pub fn draw<B: Backend>(f: &mut Frame<B>, state: &mut State) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let titles = state
        .tabs
        .titles
        .iter()
        .map(|t| {
            let (f, r) = t.split_at(1);
            Line::from(vec![
                Span::styled(
                    f,
                    Style::default()
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(r, Style::default()),
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(state.rpc_selected.clone()))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(state.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match state.tabs.index {
        0 => screens::blocks::ui::draw(f, state, chunks[1]),
        1 => screens::transactions::ui::draw(f, state, chunks[1]),
        _ => {}
    };
    if state.search_popup {
        crate::screens::search::ui::draw(f, state);
    }
    if state.rpc_list_popup {
        crate::screens::list_rpcs::ui::draw(f, state);
    }
}

