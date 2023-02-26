use reqwest::blocking::Response;
use serde_json::Value;

use crate::utils::{consts::INDEXED_NODE_URL, types::{IniitalTransactionInformation, RegistrationInformation}};

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