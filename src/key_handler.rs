
use clipboard_macos::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::global_state::{InputMode, State};
use crate::screens::blocks;

pub fn handle_key_event(state: &mut State, key: KeyEvent) {
    match state.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('b') => {
                state.set_current_tab("Blocks");
            }
            KeyCode::Char('q') => {
                state.quit();
            }
            KeyCode::Char('r') => {
                crate::screens::blocks::app::update_blocks_list(state);
            }
            KeyCode::Char('s') => {
                state.set_current_tab("Search");
            }
            KeyCode::Char('t') => {
                state.set_current_tab("Transactions");
            }
            _ => {
                if state.tabs_current == 0 {
                    blocks::keys::handle_key_event(state, key);
                }
            }
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
                // search 
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
