
use crossterm::event::{KeyCode, KeyEvent};
use crate::global_state::{InputMode, State};

pub fn handle_key_event(state: &mut State, key: KeyEvent) {
    match state.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                state.rpc_list.previous();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                state.rpc_list.next();
            }
            KeyCode::Enter => {
                let rpc = state.rpc_list.items[state.rpc_list.state.selected().unwrap()].clone();
                if rpc.0.to_string() == "CUSTOM_RPC_ENDPOINT" {
                    state.custom_rpc_popup = true;
                    state.rpc_list_popup = false;
                    return;
                }
                state.rpc_selected = rpc.1.to_string();
                state.rpc_endpoint = std::env::var(rpc.0.to_string()).unwrap();
                crate::screens::blocks::app::update_blocks_list(state);
                let block_number = state.blocks.items[state.blocks.state.selected().unwrap()].clone();
                crate::screens::blocks::app::get_block_by_number(state, &block_number);
                state.set_current_tab("Blocks");
                state.rpc_list_popup = false;
            }
            KeyCode::Esc => {
                state.rpc_list_popup = false;
            }
            _ => { }
        }
        _ => { }
    }
}
