use anyhow::Result;
use postgres::{Client, NoTls, Row};

use crate::utils::{consts::DATABASE_PATH, types::{MintRequest}};

pub fn wait_for_database() {
    let client: Result<Client> = connect_to_database();
    if client.is_err() {
        wait_for_database();
    }
}

fn connect_to_database() -> Result<Client> {
    let client: Client = Client::connect(DATABASE_PATH, NoTls)?;
    Ok(client)
}

pub fn get_mint_requests() -> Result<Vec<MintRequest>> {
    let mut client: Client = connect_to_database()?;
    let rows: Vec<Row> = client.query("SELECT * FROM mint_requests WHERE spent = false", &[])?;
    let mut mint_requests: Vec<MintRequest> = Vec::new();
    for row in rows {
        let mint_request: MintRequest = MintRequest {
            box_id: row.get(0),
            transaction_id: row.get(1)
        };
        mint_requests.push(mint_request);
    }
    Ok(mint_requests)
}

pub fn update_mint_request_to_spend(box_id: &str) {
    let mut client: Client = connect_to_database().unwrap();
    let _ = client.execute("UPDATE mint_requests SET spent = true WHERE box_id = $1", &[&box_id]);
}