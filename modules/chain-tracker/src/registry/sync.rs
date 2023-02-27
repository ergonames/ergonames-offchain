use crate::utils::{consts::INITIAL_AVLTREE_CREATION_TRANSACTION_ID, database::{write_to_confirmed_registry_insertions}, types::{convert_initial_transaction_information_to_registration_information, IniitalTransactionInformation, RegistrationInformation}, endpoints::{get_initial_transaction_information, get_mint_information}};

pub fn initial_registry_sync() {
    let initial_transaction_information: IniitalTransactionInformation = get_initial_transaction_information(INITIAL_AVLTREE_CREATION_TRANSACTION_ID);
    let initial_registration_information: RegistrationInformation = convert_initial_transaction_information_to_registration_information(initial_transaction_information);
    write_to_confirmed_registry_insertions(&initial_registration_information);
    let mut last_registration_information: RegistrationInformation = initial_registration_information;
    let mut last_spent_transaction_id: Option<String> = last_registration_information.spend_transaction_id.clone();
    while last_spent_transaction_id.is_some() {
        let registration_information: Option<RegistrationInformation> = get_mint_information(&last_registration_information.spend_transaction_id);
        if registration_information.is_some() {
            write_to_confirmed_registry_insertions(&registration_information.clone().unwrap());
            let registration_information: RegistrationInformation = registration_information.unwrap();
            last_registration_information = registration_information.clone();
            last_spent_transaction_id = registration_information.spend_transaction_id.clone();
        }
    }
}

pub fn continuous_registry_sync() {
    
}