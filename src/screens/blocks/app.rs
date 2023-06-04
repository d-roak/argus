
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
        .post(&state.rpc_endpoint)
        .json(&json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": if state.rpc_selected == "Starknet" { "starknet_blockNumber" } else { "eth_blockNumber" },
        }))
        .send()
        .unwrap();

    if res.status() != 200 {
        return;
    }

    let last_block: String = res.json::<Value>().unwrap()["result"].to_string().replace("\"", "");
    let last_block: i64 = i64::from_str_radix(&last_block[2..], 16).unwrap();

    state.blocks = StatefulList::with_items((std::cmp::max(last_block-1000,0)..last_block).rev().map(|i| i.to_string()).collect());
}

pub fn get_block_by_number(state: &mut State, block_number: &str) {
    let mut params = Vec::new();
    if state.rpc_selected == "Starknet" {
        params.push(json!({"block_number": block_number.parse::<u64>().unwrap()}));
    } else {
        let block_number = format!("0x{:x}", block_number.parse::<u64>().unwrap());
        params.push(json!(block_number));
        params.push(json!(true));
    }
    let res = Client::new()
        .post(&state.rpc_endpoint)
        .json(&json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": if state.rpc_selected == "Starknet" { "starknet_getBlockWithTxHashes" } else { "eth_getBlockByNumber" },
            "params": params
        }))
        .send()
        .unwrap();

    let block = res.json::<Value>().unwrap()["result"].clone();
    let mut ret_vec: Vec<(String, String)> = Vec::new();
    block.as_object().unwrap().iter().for_each(|(k, v)| {
        if k == "transactions" {
            let transactions = v.as_array().unwrap();
            ret_vec.push(("transactions ".to_string(), "[".to_string()));
            transactions.iter().enumerate().for_each(|(i, t)| {
                if state.rpc_selected == "Starknet" {
                    ret_vec.push((format!("  {}  ", i), format!("{}", t.to_string())));
                } else {
                    let tx = t.as_object().unwrap().clone();
                    ret_vec.push((format!("  {}  ", i), format!("{}", tx["hash"].to_string())));
                }
            });
            ret_vec.push(("".to_string(), "]".to_string()));
        } else {
            ret_vec.push(
                (format!("{} ", k.to_string()),
                format!("{}", v.to_string().replace("\"", "")))
            );
        }
    });
    state.block_info = StatefulList::with_items(ret_vec);
}

pub fn get_block_by_hash(state: &mut State, block_hash: &str) {
    let res = Client::new()
        .post(&state.rpc_endpoint)
        .json(&json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "eth_getBlockByHash",
            "params": [block_hash, true],
        }))
        .send()
        .unwrap();

    let block = res.json::<Value>().unwrap()["result"].clone();
    let mut ret_vec: Vec<(String, String)> = Vec::new();
    block.as_object().unwrap().iter().for_each(|(k, v)| {
        if k == "transactions" {
            let transactions = v.as_array().unwrap();
            ret_vec.push(("transactions".to_string(), "[".to_string()));
            transactions.iter().enumerate().for_each(|(i, t)| {
                let tx = t.as_object().unwrap().clone();
                ret_vec.push((format!("  {}  ", i), format!("{}", tx["hash"].to_string())));
            });
            ret_vec.push(("".to_string(), "]".to_string()));
        } else {
            ret_vec.push(
                (format!("{} ", k.to_string()),
                format!("{}", v.to_string().replace("\"", "")))
            );
        }
    });
    state.block_info = StatefulList::with_items(ret_vec);
}
