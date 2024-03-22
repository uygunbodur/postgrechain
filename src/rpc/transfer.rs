use crate::network::Network;
use crate::pg_extern;
use pgrx::info;
use solana_client::rpc_client::RpcClient;
use solana_sdk::bs58;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;

/// Performs a SOL transfer from one account to another on the Solana blockchain.
///
/// # Arguments
///
/// * `public_key` - The public key of the sender's account as a string. This is not used in this function but can be used for validation or logging.
/// * `secret_key` - The Base58-encoded secret key of the sender's account.
/// * `recipient_address` - The public key of the recipient's account as a string.
/// * `amount` - The amount of SOL (in lamports) to transfer. Note: 1 SOL = 1,000,000,000 lamports.
/// * `network` - The network to perform the transaction on (e.g., Mainnet, Devnet, Testnet, Localhost).
///
/// # Returns
///
/// This function returns `Ok(())` if the transfer was successful, or an error if the transfer failed.
fn sol_transfer(
    public_key: &str,
    secret_key: &str,
    to_pubkey: &str,
    amount: u64,
    network: Network,
) -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = network.get_rpc_url();
    let client = RpcClient::new(rpc_url);

    let secret_key_bytes = bs58::decode(secret_key).into_vec()?;
    let sender_keypair = Keypair::from_bytes(&secret_key_bytes)?;

    let recipient_pubkey = Pubkey::from_str(to_pubkey)?;

    let transfer_instruction =
        system_instruction::transfer(&sender_keypair.pubkey(), &recipient_pubkey, amount);

    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&sender_keypair.pubkey()),
        &[&sender_keypair],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;
    info!("Transaction:{}", signature);
    Ok(())
}

#[pg_extern]
fn pc_transfer(
    public_key: &str,
    secret_key: &str,
    to_pubkey: &str,
    amount: i64,
    network_str: &str,
) -> bool {
    let network = Network::from_str(network_str);
    match sol_transfer(
        public_key,
        secret_key,
        to_pubkey,
        (amount * 1_000_000_000).try_into().unwrap(),
        network,
    ) {
        Ok(_) => true,
        Err(e) => {
            info!("Transfer Failed: {:?}", e);
            false
        }
    }
}
