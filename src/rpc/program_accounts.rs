use crate::network::Network;
use pgrx::prelude::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::str::FromStr;

pub struct AccountInfo {
    public_key: String,
    data_len: i64,
    lamports: i64,
    rent_epoch: i64,
    executable: bool,
}

/// Fetches a list of accounts associated with a given program ID on the specified network.
///
/// This function connects to the Solana blockchain via the provided network's RPC URL,
/// and retrieves all accounts that are owned by the specified program ID. Each account's
/// information, including its public key, the size of its data, the amount of lamports it holds,
/// the rent epoch it is in, and its executable flag, is then encapsulated into an `AccountInfo`
/// struct. This allows for easy access and manipulation of account data returned by the query.
///
/// # Arguments
/// * `program_id` - A string slice that holds the program ID whose accounts are to be fetched.
/// * `network` - An instance of the `Network` enum specifying which Solana network (e.g., mainnet, testnet) to query.
///
/// # Returns
/// A `Result` containing either:
/// - On success: a `Vec<AccountInfo>` where each `AccountInfo` represents an account associated with the `program_id`.
/// - On failure: a boxed error (`Box<dyn std::error::Error>`) indicating what went wrong during the function's execution.
///
/// # Errors
/// This function can return an error in several cases, including but not limited to:
/// - Network connection issues.
/// - Invalid program ID format.
/// - Problems parsing the returned account data.
///
/// # Example
/// ```
/// let program_id = "ExampleProgramId123";
/// let network = Network::Mainnet;
/// match get_program_accounts(program_id, network) {
///     Ok(accounts) => println!("Found {} accounts associated with the program ID.", accounts.len()),
///     Err(e) => println!("An error occurred: {}", e),
/// }
/// ```
///
/// Note: This function relies on the Solana client (`solana_client`) library to make RPC calls to the Solana network.
pub fn get_program_accounts(
    program_id: &str,
    network: Network,
) -> Result<Vec<AccountInfo>, Box<dyn std::error::Error>> {
    let rpc_url = network.get_rpc_url();
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
    let program_pubkey = Pubkey::from_str(program_id)?;

    let accounts = client.get_program_accounts(&program_pubkey)?;
    let mut results = Vec::new();
    for (pubkey, account) in accounts.iter() {
        results.push(AccountInfo {
            public_key: pubkey.to_string(),
            data_len: account.data.len().try_into().unwrap(),
            lamports: account.lamports.try_into().unwrap(),
            rent_epoch: account.rent_epoch.try_into().unwrap(),
            executable: account.executable,
        });
    }
    Ok(results)
}

#[pg_extern]
fn pc_get_program_accounts(
    program_id: &str,
    network_str: &str,
) -> Result<
    TableIterator<
        'static,
        (
            name!(public_key, String),
            name!(data_len, i64),
            name!(lamports, i64),
            name!(rent_epoch, i64),
            name!(executable, bool),
        ),
    >,
    String,
> {
    let network = Network::from_str(network_str);
    let results: Vec<(String, i64, i64, i64, bool)> = Vec::new();
    match get_program_accounts(program_id, network) {
        Ok(account_infos) => {
            let results: Vec<(String, i64, i64, i64, bool)> = account_infos
                .into_iter()
                .map(|account_info| {
                    (
                        account_info.public_key,
                        account_info.data_len,
                        account_info.lamports,
                        account_info.rent_epoch,
                        account_info.executable,
                    )
                })
                .collect();
            Ok(TableIterator::new(results))
        }
        Err(e) => Err(e.to_string()),
    }
}
