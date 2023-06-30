
use clipboard_macos::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::global::state::{InputMode, State};

pub fn handle_key_event(state: &mut State, key: KeyEvent) {
    match state.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('i') => {
                state.set_input_mode("insert".to_string());
            }
            KeyCode::Esc => {
                state.set_input_mode("normal".to_string());
                state.input_buffer = "".to_string();
                state.custom_rpc_popup = false;
            }
            _ => { }
        }
        InputMode::Insert => match key.code {
            KeyCode::Char(c) => {
                if c == 'v' && key.modifiers == KeyModifiers::CONTROL {
                    let clipboard = Clipboard::new().unwrap();
                    let clipboard_text = clipboard.read().unwrap();
                    state.input_buffer.push_str(&clipboard_text);
                }
                state.input_buffer.push(c);
            }
            KeyCode::Enter => {
                state.rpc_endpoint = state.input_buffer.clone();
                state.rpc_selected = "Custom RPC".to_string();
                
                crate::screens::blocks::app::update_blocks_list(state);
                let block_number = state.blocks.items[state.blocks.state.selected().unwrap()].clone();
                crate::screens::blocks::app::get_block_by_number(state, &block_number);
                state.set_current_tab("Blocks");

                state.set_input_mode("normal".to_string());
                state.input_buffer = "".to_string();
                state.custom_rpc_popup = false;
            }
            KeyCode::Backspace => {
                state.input_buffer.pop();
            }
            KeyCode::Esc => {
                state.set_input_mode("normal".to_string());
            }
            _ => {}
        }
    }
}
