use crate::utils::{consts::PROXY_CONTRACT_ERGOTREE, endpoints::get_mint_requests_at_proxy_address, types::MintRequest};

pub fn track_mempool() {
    let mint_requests: Option<Vec<MintRequest>> = get_mint_requests_at_proxy_address(PROXY_CONTRACT_ERGOTREE);
    if mint_requests.is_none() {
        return;
    }
    let mint_requests: Vec<MintRequest> = mint_requests.unwrap();
    for mint_request in mint_requests {
        println!("{:?}", mint_request);
    }
}