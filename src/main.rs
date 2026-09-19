use base64::{Engine as _, engine::general_purpose};
use std::fs;

fn main() {
    let cookie_path = "/home/zerum/.bitcoin-mainnet-pruned/.cookie";

    let cookie = match fs::read_to_string(cookie_path) {
        Ok(c) => c.trim().to_string(),
        Err(e) => {
            println!("Failed to read cookie: {}", e);
            return;
        }
    };

    let auth = general_purpose::STANDARD.encode(&cookie);

    let client = reqwest::blocking::Client::new();
    let body = r#"{"jsonrpc":"1.0","id":"fee_check",
                "method":"getblockchaininfo","params":[]}"#;

    let resp = match client
        .post("http://127.0.0.1:8332/")
        .header("Authorization", format!("Basic {}", auth))
        .body(body)
        .send()
    {
        Ok(r) => r,
        Err(e) => {
            println!("Request error: {}", e);
            return;
        }
    };

    let json: serde_json::Value = match resp.json() {
        Ok(j) => j,
        Err(e) => {
            println!("Failed to parse json: {}", e);
            return;
        }
    };

    println!("blocks: {}", json["result"]["blocks"]);
    println!("chain: {}", json["result"]["chain"]);
}
