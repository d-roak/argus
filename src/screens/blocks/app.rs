
use serde_json::{ json, Value };
use reqwest::blocking::Client;

use crate::global_state::{State, StatefulList};

#[derive(Debug)]
pub struct Block {
    pub height: u64,
    pub hash: String,
    pub time: u64,
}

pub fn update_blocks_list(state: &mut State) {
    let res = Client::new()
        .post("")
        .json(&json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
        }))
        .send()
        .unwrap();

    if res.status() != 200 {
        return;
    }

    let last_block: String = res.json::<Value>().unwrap()["result"].to_string().replace("\"", "");
    let last_block: i64 = i64::from_str_radix(&last_block[2..], 16).unwrap();

    state.blocks = StatefulList::with_items((last_block-1000..last_block).rev().map(|i| i.to_string()).collect());
}

pub fn get_block_by_number(state: &mut State, block_number: &str) {
    let block_number = format!("0x{:x}", block_number.parse::<u64>().unwrap());
    let res = Client::new()
        .post("")
        .json(&json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": [block_number, true],
        }))
        .send()
        .unwrap();

    let block = res.json::<Value>().unwrap()["result"].clone();

    state.block_info = StatefulList::with_items(block.as_object().unwrap().iter().map(|(k, v)| (k.to_string(), v.to_string())).collect());
}
