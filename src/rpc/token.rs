use crate::network::Network;
use pgrx::prelude::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Token hesabının bakiyesini sorgular.
///
/// # Arguments
///
/// * `rpc_url` - Solana RPC sunucusunun URL'si.
/// * `token_account_address` - Sorgulanacak token hesabının adresi.
///
pub fn get_token_balance(
    token_account_address: &str,
    network: Network,
) -> Result<u64, Box<dyn std::error::Error>> {
    let rpc_url = network.get_rpc_url();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let token_account_pubkey = Pubkey::from_str(token_account_address)?;

    let balance = client.get_token_account_balance(&token_account_pubkey)?;
    Ok(balance.ui_amount.unwrap_or(0.0) as u64)
}

#[pg_extern]
fn pc_token_account_balance(token_account_address: &str, network_str: &str) -> AnyNumeric {
    let network = Network::from_str(network_str);
    match get_token_balance(token_account_address, network) {
        Ok(balance) => AnyNumeric::from(balance),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            AnyNumeric::from(0)
        }
    }
}
