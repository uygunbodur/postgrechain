use crate::errors::balance::BalanceError;
use crate::network::Network;
use pgrx::prelude::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Queries the balance of a given Solana account address on the specified network.
///
/// # Arguments
///
/// * `account_address` - A string slice that holds the Solana account address.
/// * `network` - The Solana network to query the balance from (e.g., Mainnet, Devnet).
///
/// # Examples
///
/// ```
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let balance = solana_sdk_rust_example::get_balance("Bs53MuEQA5yqJNfwLWqBHf2QcixNAkPgQ6XqsZ9kaXbL", solana_sdk_rust_example::network::Network::Devnet)?;
/// println!("Balance is: {}", balance);
/// # Ok(())
/// # }
/// ```
pub fn get_balance(account_address: &str, network: Network) -> Result<u64, BalanceError> {
    let rpc_url = network.get_rpc_url();
    let client = RpcClient::new(rpc_url);
    let account_pubkey = Pubkey::from_str(account_address)
        .map_err(|e| BalanceError::InvalidAddress(e.to_string()))?;
    client
        .get_balance(&account_pubkey)
        .map_err(|ee| BalanceError::RpcError(ee.to_string()))
}

#[pg_extern]
fn pc_balance(account_address: &str, network_str: &str) -> AnyNumeric {
    let network = Network::from_str(network_str);
    match get_balance(account_address, network) {
        Ok(balance) => AnyNumeric::from(balance),
        Err(BalanceError::RpcError(e)) => {
            info!("RPC Error: {}", e.to_string());
            AnyNumeric::from(0)
        }
        Err(BalanceError::InvalidAddress(e)) => {
            info!("Address Error: {}", e.to_string());
            AnyNumeric::from(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::Network;

    #[test]
    fn test_get_balance() {
        let result = get_balance(
            "Bs53MuEQA5yqJNfwLWqBHf2QcixNAkPgQ6XqsZ9kaXbL",
            Network::Devnet,
        )
        .unwrap();
        assert_ne!(result, 0, "");
    }
}
