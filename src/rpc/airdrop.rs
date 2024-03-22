use crate::network::Network;
use crate::pg_extern;
use pgrx::info;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Airdrops `amount` SOL to the specified `account_address` on the specified `network`.
///
/// # Arguments
///
/// * `network_url` - The URL of the Solana RPC server (e.g., devnet or testnet).
/// * `account_address` - The address of the account to receive the airdrop.
/// * `amount` - The amount of SOL to airdrop, in lamports (1 SOL = 1,000,000,000 lamports).
///
pub fn airdrop_sol(
    network: Network,
    account_address: &str,
    amount: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = network.get_rpc_url();
    let client = RpcClient::new(rpc_url);
    let account_pubkey = Pubkey::from_str(account_address)?;

    let signature = client.request_airdrop(&account_pubkey, amount)?;
    client.confirm_transaction(&signature)?;
    info!("Transaction:{}", signature);
    Ok(())
}

#[pg_extern]
fn pc_airdrop(account_address: &str, amount: i64, network_str: &str) -> bool {
    let network = Network::from_str(network_str);
    match airdrop_sol(
        network,
        account_address,
        (amount * 1_000_000_000).try_into().unwrap(),
    ) {
        Ok(_) => true,
        Err(e) => {
            info!("Airdrop failed: {:?}", e);
            false
        }
    }
}
