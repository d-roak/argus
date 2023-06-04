
use crossterm::event::{KeyCode, KeyEvent};
use crate::global_state::{InputMode, State};
use crate::screens;

pub fn handle_key_event(state: &mut State, key: KeyEvent) {
    match state.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('\'') => {
                if state.rpc_list_popup {
                    state.rpc_list_popup = false;
                } else {
                    state.rpc_list_popup = true;
                }
            }
            KeyCode::Char('b') => {
                state.set_current_tab("Blocks");
            }
            KeyCode::Char('q') => {
                state.quit();
            }
            KeyCode::Char('r') => {
                crate::screens::blocks::app::update_blocks_list(state);
            }
            KeyCode::Char('t') => {
                state.set_current_tab("Transactions");
            }
            _ => {
                if state.search_popup {
                    screens::search::keys::handle_key_event(state, key);
                } else if state.rpc_list_popup {
                    screens::list_rpcs::keys::handle_key_event(state, key);
                } else if state.custom_rpc_popup {
                    screens::custom_rpc::keys::handle_key_event(state, key);
                } else if state.tabs_current == 0 {
                    screens::blocks::keys::handle_key_event(state, key);
                } else if state.tabs_current == 1 {
                    screens::transactions::keys::handle_key_event(state, key);
                }
            }
        }
        _ => {
            if state.search_popup {
                screens::search::keys::handle_key_event(state, key);
            } else if state.custom_rpc_popup {
                screens::custom_rpc::keys::handle_key_event(state, key);
            }
        }
    }
}
