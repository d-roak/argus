use crate::{app::App, global_state::State};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Borders, Block, Tabs,
    },
    Frame,
};
use crate::screens;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App, state: &mut State) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let titles = state
        .tabs
        .titles
        .iter()
        .map(|t| {
            let (f, r) = t.split_at(1);
            Spans::from(vec![
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
        .block(Block::default().borders(Borders::ALL).title(app.title))
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
}

