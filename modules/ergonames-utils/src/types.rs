#[derive(Clone, Debug)]
pub struct RegistrationInformation {
    pub ergoname_registered: String,
    pub mint_transaction_id: String,
    pub spend_transaction_id: Option<String>,
    pub ergoname_token_id: String,
}

#[derive(Clone, Debug)]
pub struct IniitalTransactionInformation {
    pub transaction_id: String,
    pub spent_transaction_id: Option<String>
}

#[derive(Clone, Debug)]
pub struct MintRequest {
    pub transaction_id: String,
    pub box_id: String
}

pub fn convert_initial_transaction_information_to_registration_information(initial_transaction_information: IniitalTransactionInformation) -> RegistrationInformation {
    let ri: RegistrationInformation = RegistrationInformation {
        ergoname_registered: "".to_string(),
        mint_transaction_id: initial_transaction_information.transaction_id,
        spend_transaction_id: initial_transaction_information.spent_transaction_id,
        ergoname_token_id: "".to_string(),
    };
    ri
}