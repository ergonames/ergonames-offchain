use std::{thread, time};
use anyhow::Result;

pub mod utils;

use utils::{database::{get_mint_requests, update_mint_request_to_spend}, types::MintRequest};

const LVIE_MODE: &str = "true";

fn main() {
    loop {
        let mint_requests: Vec<MintRequest> = get_mint_requests().unwrap();
        println!("Found {} mint requests", mint_requests.len());
        for req in mint_requests {
            let box_id_to_spend: &str = &req.box_id;
            println!("Spending box: {}", box_id_to_spend);
            let output = run_jar(box_id_to_spend, LVIE_MODE);
            println!("{:?}", output);
            update_mint_request_to_spend(box_id_to_spend);
            thread::sleep(time::Duration::from_secs(2));
        }
    }
}

fn run_jar(box_id: &str, live_mode: &str) -> Result<String> {
    let opt = std::process::Command::new("java")
        .args(&["-jar", "ergonames-transaction-utils.jar", box_id, live_mode])
        .output()
        .expect("Failed to run ergonames-transaction-utils.jar");

    let output = String::from_utf8_lossy(&opt.stdout);
    if output.contains("Error") {
        return Err(anyhow::anyhow!("Error"));
    }
    Ok(output.to_string())
}