
use crossterm::event::{KeyCode, KeyEvent};
use crate::{global_state::State, screens::transactions::app::get_tx_by_hash};

use super::app::get_block_by_number;

pub fn handle_key_event(state: &mut State, key: KeyEvent) {
    let focus = state
        .blocks_focus
        .items[state.blocks_focus.state.selected().unwrap()].clone();
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            if focus == "last_blocks" {
                state.blocks.previous();
                let block_number = state.blocks.items[state.blocks.state.selected().unwrap()].clone();
                get_block_by_number(state, &block_number);
            } else if focus == "block_info" {
                state.block_info.previous();
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if focus == "last_blocks" {
                state.blocks.next();
                let block_number = state.blocks.items[state.blocks.state.selected().unwrap()].clone();
                get_block_by_number(state, &block_number);
            } else if focus == "block_info" {
                state.block_info.next();
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            state.blocks_focus.previous();
        }
        KeyCode::Right | KeyCode::Char('l') => {
            state.blocks_focus.next();
        }
        KeyCode::Enter => {
            if focus == "last_blocks" {
                state.blocks_focus.next();
            } else if focus == "block_info" {
                let tx_hash = state.block_info.items[state.block_info.state.selected().unwrap()].1.clone().replace("\"", "");
                get_tx_by_hash(state, &tx_hash);
                state.set_current_tab("Transactions");
            }
        }
        _ => {}
    }
}
