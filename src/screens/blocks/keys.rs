
use clipboard_macos::Clipboard;
use crossterm::event::{KeyCode, KeyEvent};
use crate::{global_state::State, screens::transactions::app::get_tx_by_hash};

use super::app::{get_block_by_number, get_block_by_hash};

pub fn handle_key_event(state: &mut State, key: KeyEvent) {
    let focus = state
        .focus
        .items[state.focus.state.selected().unwrap()].clone();
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
            state.focus.previous();
        }
        KeyCode::Right | KeyCode::Char('l') => {
            state.focus.next();
        }
        KeyCode::Char('d') => {
            for (k, v) in state.block_info.items.iter_mut() {
                if k.trim() == "baseFeePerGas" {
                    if v.starts_with("0x") {
                        let decoded = u64::from_str_radix(v.trim_start_matches("0x"), 16).unwrap();
                        *v = decoded.to_string();
                    } else {
                        let encoded = format!("0x{:x}", v.parse::<u64>().unwrap());
                        *v = encoded;
                    }
                } else if k.trim() == "difficulty" {
                    if v.starts_with("0x") {
                        let decoded = u64::from_str_radix(v.trim_start_matches("0x"), 16).unwrap();
                        *v = decoded.to_string();
                    } else {
                        let encoded = format!("0x{:x}", v.parse::<u64>().unwrap());
                        *v = encoded;
                    }
                } else if k.trim() == "gasLimit" {
                    if v.starts_with("0x") {
                        let decoded = u64::from_str_radix(v.trim_start_matches("0x"), 16).unwrap();
                        *v = decoded.to_string();
                    } else {
                        let encoded = format!("0x{:x}", v.parse::<u64>().unwrap());
                        *v = encoded;
                    }
                } else if k.trim() == "gasUsed" {
                    if v.starts_with("0x") {
                        let decoded = u64::from_str_radix(v.trim_start_matches("0x"), 16).unwrap();
                        *v = decoded.to_string();
                    } else {
                        let encoded = format!("0x{:x}", v.parse::<u64>().unwrap());
                        *v = encoded;
                    }
                } else if k.trim() == "nonce" {
                    if v.starts_with("0x") {
                        let decoded = u64::from_str_radix(v.trim_start_matches("0x"), 16).unwrap();
                        *v = decoded.to_string();
                    } else {
                        let encoded = format!("0x{:x}", v.parse::<u64>().unwrap());
                        *v = encoded;
                    }
                } else if k.trim() == "number" {
                    if v.starts_with("0x") {
                        let decoded = u64::from_str_radix(v.trim_start_matches("0x"), 16).unwrap();
                        *v = decoded.to_string();
                    } else {
                        let encoded = format!("0x{:x}", v.parse::<u64>().unwrap());
                        *v = encoded;
                    }
                } else if k.trim() == "size" {
                    if v.starts_with("0x") {
                        let decoded = u64::from_str_radix(v.trim_start_matches("0x"), 16).unwrap();
                        *v = decoded.to_string();
                    } else {
                        let encoded = format!("0x{:x}", v.parse::<u64>().unwrap());
                        *v = encoded;
                    }
                } else if k.trim() == "timestamp" {
                    if v.starts_with("0x") {
                        let decoded = u64::from_str_radix(v.trim_start_matches("0x"), 16).unwrap();
                        let iso = chrono::NaiveDateTime::from_timestamp_opt(decoded as i64, 0);
                        if let Some(iso) = iso {
                            *v = iso.to_string();
                        }
                    } else {
                        let iso = chrono::NaiveDateTime::parse_from_str(v, "%Y-%m-%d %H:%M:%S").unwrap();
                        let encoded = format!("0x{:x}", iso.timestamp());
                        *v = encoded;
                    }
                } else if k.trim() == "totalDifficulty" {
                    if v.starts_with("0x") {
                        let decoded = u128::from_str_radix(v.trim_start_matches("0x"), 16).unwrap();
                        *v = decoded.to_string();
                    } else {
                        let encoded = format!("0x{:x}", v.parse::<u128>().unwrap());
                        *v = encoded;
                    }
                }
            }
        }
        KeyCode::Char('s') => {
            if state.search_popup {
                state.search_popup = false;
            } else {
                state.search_popup = true;
            }
        }
        KeyCode::Enter => {
            if focus == "last_blocks" {
                state.focus.next();
            } else if focus == "block_info" {
                let tuple = state.block_info.items[state.block_info.state.selected().unwrap()].clone();
                if tuple.0.trim().parse::<u32>().is_ok() {
                    let tx_hash = tuple.1.replace("\"", "");
                    get_tx_by_hash(state, &tx_hash);
                    state.set_current_tab("Transactions");
                } else if tuple.0.trim() == "parentHash" {
                    get_block_by_hash(state, &tuple.1);
                } else {
                    let mut clipboard = Clipboard::new().unwrap();
                    clipboard.write(tuple.1.replace("\"", "")).unwrap();
                }
            }
        }
        _ => {}
    }
}
