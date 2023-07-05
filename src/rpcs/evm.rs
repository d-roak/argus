use serde_json::{ json, Value };
use reqwest::blocking::Client;

use crate::global::state::State;

pub fn get_block_number(state: &mut State) -> i64 {
    let res = Client::new()
        .post(&state.rpc_endpoint)
        .json(&json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
        }))
        .send()
        .unwrap();

    if res.status() != 200 {
        return -1;
    }

    let last_block: String = res.json::<Value>().unwrap()["result"].to_string().replace("\"", "");
    i64::from_str_radix(&last_block[2..], 16).unwrap()
}


