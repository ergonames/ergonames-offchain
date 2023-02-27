use anyhow::Result;
use postgres::{Client, NoTls, Row, Error};

use crate::utils::{consts::DATABASE_PATH, types::{RegistrationInformation, MintRequest}};

pub fn wait_for_database() {
    let client: Result<Client> = connect_to_database();
    if client.is_err() {
        wait_for_database();
    }
}

pub fn write_to_confirmed_registry_insertions(registration_information: &RegistrationInformation) {
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

pub fn write_to_mint_requests(mint_request: &MintRequest) {
    let mut client: Client = connect_to_database().unwrap_or_else(|e| {
        eprintln!("Error connecting to database: {}", e);
        std::process::exit(1);
    });

    let query = "INSERT INTO mint_requests (
        box_id,
        transaction_id
    ) VALUES ($1, $2); ";
    client.execute(query, &[
        &mint_request.box_id,
        &mint_request.transaction_id,
    ]).unwrap();
}

pub fn create_database_schema() {
    create_registration_information_schema();
    create_mint_requests_schema();
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

fn create_mint_requests_schema() {
    let mut client: Client = connect_to_database().unwrap_or_else(|e| {
        eprintln!("Error connecting to database: {}", e);
        std::process::exit(1);
    });
    let query: &str = "CREATE TABLE IF NOT EXISTS mint_requests (
        box_id VARCHAR(64) NOT NULL PRIMARY KEY,
        transaction_id VARCHAR(64) NOT NULL
    );";

    client.execute(query, &[]).unwrap();
}

fn connect_to_database() -> Result<Client> {
    let client: Client = Client::connect(DATABASE_PATH, NoTls)?;
    Ok(client)
}

pub fn get_last_confirmed_registry_insertion() -> RegistrationInformation {
    let mut client: Client = connect_to_database().unwrap_or_else(|e| {
        eprintln!("Error connecting to database: {}", e);
        std::process::exit(1);
    });

    let query: &str = "SELECT * FROM confirmed_registry_insertions WHERE spend_transaction_id IS NULL";
    let rows: Vec<Row> = client.query(query, &[]).unwrap();
    let row: Option<&Row> = rows.get(0);
    if row.is_none() {
        panic!("No last confirmed registry insertion found");
    }
    let row: &Row = row.unwrap();
    let ergoname_registered: String = row.get(0);
    let mint_transaction_id: String = row.get(1);
    let mut spend_transaction_id: Option<String> = None;
    let spend_transaction_id_raw: Result<Option<String>, Error> = row.try_get(2);
    if spend_transaction_id_raw.is_ok() {
        spend_transaction_id = spend_transaction_id_raw.unwrap();
    }
    let ergoname_token_id: String = row.get(3);
    let registration_information: RegistrationInformation = RegistrationInformation {
        ergoname_registered,
        mint_transaction_id,
        spend_transaction_id,
        ergoname_token_id,
    };
    registration_information
}