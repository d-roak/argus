
use clipboard_macos::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::global_state::{InputMode, State};

pub fn handle_key_event(state: &mut State, key: KeyEvent) {
    match state.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('i') => {
                state.set_input_mode("insert".to_string());
            }
            KeyCode::Char('s') | KeyCode::Esc => {
                state.set_input_mode("normal".to_string());
                state.input_buffer = "".to_string();
                state.search_popup = false;
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
                if state.tabs_current == 0 {
                    let block_input = state.input_buffer.clone();
                    if block_input.parse::<u32>().is_ok() {
                        crate::screens::blocks::app::get_block_by_number(state, &block_input);
                    } else {
                        crate::screens::blocks::app::get_block_by_hash(state, &block_input);
                    }
                    state.set_input_mode("normal".to_string());
                    state.input_buffer = "".to_string();
                    state.search_popup = false;
                } else if state.tabs_current == 1 {
                    let tx_hash = state.input_buffer.clone();
                    crate::screens::transactions::app::get_tx_by_hash(state, &tx_hash);
                    state.set_input_mode("normal".to_string());
                    state.input_buffer = "".to_string();
                    state.search_popup = false;
                }
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
