
use clipboard_macos::Clipboard;
use crossterm::event::{KeyCode, KeyEvent};
use crate::global::state::State;


pub fn handle_key_event(state: &mut State, key: KeyEvent) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            state.tx_info.previous();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            state.tx_info.next();
        }
        KeyCode::Char('s') => {
            if state.search_popup {
                state.search_popup = false;
            } else {
                state.search_popup = true;
            }
        }
        KeyCode::Enter => {
            let value = state.tx_info.items[state.tx_info.state.selected().unwrap()].1.clone().replace("\"", "");
            let mut clipboard = Clipboard::new().unwrap();
            clipboard.write(value).unwrap();
        }
        _ => {}
    }
}
