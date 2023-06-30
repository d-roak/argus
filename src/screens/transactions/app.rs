use serde_json::{ json, Value };
use reqwest::blocking::Client;

use crate::global::state::{State, StatefulList};

#[derive(Debug)]
pub struct Block {
    pub height: u64,
    pub hash: String,
    pub time: u64,
}

pub fn get_tx_by_hash(state: &mut State, tx_hash: &str) {
    let res = Client::new()
        .post(&state.rpc_endpoint)
        .json(&json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": if state.rpc_selected == "Starknet" { "starknet_getTransactionByHash" } else { "eth_getTransactionByHash" },

            "params": [tx_hash],
        }))
        .send()
        .unwrap();
    
    let tx = res.json::<Value>().unwrap()["result"].clone();
    
    let mut ret_vec: Vec<(String, String)> = Vec::new();
    tx.as_object().unwrap().iter().for_each(|(k, v)| {
        ret_vec.push((k.to_string(), v.to_string()));
    });
    state.tx_info = StatefulList::with_items(ret_vec);
}
