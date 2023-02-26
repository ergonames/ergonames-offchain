use anyhow::Result;
use postgres::{Client, NoTls};
use reqwest::blocking::Response;
use serde_json::Value;

const DATABASE_PATH: &str = "postgresql://ergonames:ergonames@localhost:5432/ergonames";
const INITIAL_AVLTREE_CREATION_TRANSACTION_ID: &str = "e271e7cb9b9c7932546e8a5746c91cb1c0f1114ff173a90e1fe979170f71c579";
const INDEXED_NODE_URL: &str = "http://198.58.96.195:9052";

#[derive(Clone, Debug)]
struct RegistrationInformation {
    ergoname_registered: String,
    mint_transaction_id: String,
    spend_transaction_id: Option<String>,
    ergoname_token_id: String,
}

#[derive(Clone, Debug)]
struct IniitalTransactionInformation {
    transaction_id: String,
    spent_transaction_id: Option<String>
}

fn main() {
    create_database_schema();
    let initial_transaction_information: IniitalTransactionInformation = get_initial_transaction_information(INITIAL_AVLTREE_CREATION_TRANSACTION_ID);
    let initial_registration_information: RegistrationInformation = convert_initial_transaction_information_to_registration_information(initial_transaction_information);
    write_to_confirmed_registry_insertions(&initial_registration_information);
    let mut last_registration_information: RegistrationInformation = initial_registration_information;
    loop {
        let registration_information: Option<RegistrationInformation> = get_mint_information(&last_registration_information.spend_transaction_id);
        if registration_information.is_some() {
            write_to_confirmed_registry_insertions(&registration_information.clone().unwrap());
            let registration_information: RegistrationInformation = registration_information.unwrap();
            last_registration_information = registration_information.clone();
        }
    }
}

fn connect_to_database() -> Result<Client> {
    let client: Client = Client::connect(DATABASE_PATH, NoTls)?;
    Ok(client)
}

fn create_database_schema() {
    create_registration_information_schema();
}

fn create_registration_information_schema() {
    let mut client: Client = connect_to_database().unwrap_or_else(|e| {
        eprintln!("Error connecting to database: {}", e);
        std::process::exit(1);
    });

    let query: &str = "CREATE TABLE IF NOT EXISTS confirmed_registry_insertions (
        ergoname_registered VARCHAR(64) NOT NULL PRIMARY KEY,
        mint_transaction_id VARCHAR(64) NOT NULL,
        spend_transaction_id VARCHAR(64),
        ergoname_token_id VARCHAR(64) NOT NULL
    );";

    client.execute(query, &[]).unwrap();
}

fn write_to_confirmed_registry_insertions(registration_information: &RegistrationInformation) {
    let mut client: Client = connect_to_database().unwrap_or_else(|e| {
        eprintln!("Error connecting to database: {}", e);
        std::process::exit(1);
    });

    let query = "INSERT INTO confirmed_registry_insertions (
        ergoname_registered,
        mint_transaction_id,
        spend_transaction_id,
        ergoname_token_id
    ) VALUES ($1, $2, $3, $4); ";
    client.execute(query, &[
        &registration_information.ergoname_registered,
        &registration_information.mint_transaction_id,
        &registration_information.spend_transaction_id,
        &registration_information.ergoname_token_id,
    ]).unwrap();
}

fn get_initial_transaction_information(initial_transaction_id: &str) -> IniitalTransactionInformation {
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

fn get_mint_information(last_spent_transaction_id: &Option<String>) -> Option<RegistrationInformation> {
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

fn convert_initial_transaction_information_to_registration_information(initial_transaction_information: IniitalTransactionInformation) -> RegistrationInformation {
    let ri: RegistrationInformation = RegistrationInformation {
        ergoname_registered: "".to_string(),
        mint_transaction_id: initial_transaction_information.transaction_id,
        spend_transaction_id: initial_transaction_information.spent_transaction_id,
        ergoname_token_id: "".to_string(),
    };
    ri
}