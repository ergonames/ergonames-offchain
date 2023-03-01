use reqwest::blocking::{Response, Client};
use serde_json::Value;

use crate::{consts::INDEXED_NODE_URL, types::{IniitalTransactionInformation, MintRequest, RegistrationInformation}};

pub fn get_initial_transaction_information(initial_transaction_id: &str) -> IniitalTransactionInformation {
    let url: String = format!("{}/blockchain/transaction/byId/{}", INDEXED_NODE_URL, initial_transaction_id);
    let response: Response = reqwest::blocking::get(&url).unwrap();
    let body: String = response.text().unwrap();
    let body: Value = serde_json::from_str(&body).unwrap();
    let transaction_id: &str = body["id"].as_str().unwrap();
    let spend_transaction_id: Option<&str> = body["outputs"][0]["spentTransactionId"].as_str();
    let initial_transaction_information: IniitalTransactionInformation = IniitalTransactionInformation {
        transaction_id: transaction_id.to_string(),
        spent_transaction_id: spend_transaction_id.map(|s| s.to_string())
    };
    initial_transaction_information
}

pub fn get_mint_information(last_spent_transaction_id: &Option<String>) -> Option<RegistrationInformation> {
    if last_spent_transaction_id.is_none() {
        return None;
    }
    let url: String = format!("{}/blockchain/transaction/byId/{}", INDEXED_NODE_URL, last_spent_transaction_id.clone().unwrap());
    let response: Response = reqwest::blocking::get(&url).unwrap();
    let body: String = response.text().unwrap();
    let body: Value = serde_json::from_str(&body).unwrap();
    let ergoname_raw: &str = body["outputs"][0]["additionalRegisters"]["R4"].as_str().unwrap();
    let ergoname_raw: &str = &ergoname_raw[4..];
    let ergoname_bytes: Vec<u8> = hex::decode(ergoname_raw).unwrap();
    let ergoname_registered: String = String::from_utf8(ergoname_bytes).unwrap();
    let mint_transaction_id: &str = body["id"].as_str().unwrap();
    let spend_transaction_id: Option<String> = body["outputs"][1]["spentTransactionId"].as_str().map(|s| s.to_string());
    let ergoname_token_id: &str = body["outputs"][0]["assets"][0]["tokenId"].as_str().unwrap();
    let registration_information: RegistrationInformation = RegistrationInformation {
        ergoname_registered: ergoname_registered.to_string(),
        mint_transaction_id: mint_transaction_id.to_string(),
        spend_transaction_id: spend_transaction_id,
        ergoname_token_id: ergoname_token_id.to_string(),
    };
    Some(registration_information)
}

pub fn get_mint_requests_at_proxy_address(ergotree: &str) -> Option<Vec<MintRequest>> {
    let bod: String = ergotree.clone().to_string();
    let url: String = format!("{}/blockchain/box/unspent/byErgoTree", INDEXED_NODE_URL);
    let client: Client = reqwest::blocking::Client::new();
    let response: Response = client.post(&url)
        .body(bod)
        .send()
        .unwrap();
    let body: String = response.text().unwrap();
    let body: Value = serde_json::from_str(&body).unwrap();
    let transactions: Vec<Value> = body.as_array().unwrap().to_vec();
    let mut mint_requests: Vec<MintRequest> = Vec::new();
    for transaction in transactions {
        let transaction_id: &str = transaction["transactionId"].as_str().unwrap();
        let box_id: &str = transaction["boxId"].as_str().unwrap();
        let mint_request: MintRequest = MintRequest {
            transaction_id: transaction_id.to_string(),
            box_id: box_id.to_string()
        };
        mint_requests.push(mint_request);
    }
    if mint_requests.len() == 0 {
        return None;
    }
    Some(mint_requests)
}