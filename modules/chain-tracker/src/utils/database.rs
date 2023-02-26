use anyhow::Result;
use postgres::{Client, NoTls};

use crate::utils::{consts::DATABASE_PATH, types::RegistrationInformation};

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

pub fn create_database_schema() {
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

fn connect_to_database() -> Result<Client> {
    let client: Client = Client::connect(DATABASE_PATH, NoTls)?;
    Ok(client)
}

