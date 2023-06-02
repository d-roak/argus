
use crossterm::event::{KeyCode, KeyEvent};
use crate::global_state::State;

use super::app::get_block_by_number;

pub fn handle_key_event(state: &mut State, key: KeyEvent) {
    let focus = state
        .blocks_focus
        .items[state.blocks_focus.state.selected().unwrap()].clone();
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if focus == "last_blocks" {
                state.blocks.previous();
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if focus == "last_blocks" {
                state.blocks.next();
                let block_number = state.blocks.items[state.blocks.state.selected().unwrap()].clone();
                get_block_by_number(state, &block_number);
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            state.blocks_focus.previous();
        }
        KeyCode::Right | KeyCode::Char('l') => {
            state.blocks_focus.next();
        }
        KeyCode::Enter => {
        }
        _ => {}
    }
}
