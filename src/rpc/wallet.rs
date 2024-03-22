use pgrx::prelude::*;
use solana_sdk::bs58;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

/// Creates a new Solana wallet and returns the public key and secret key as strings.
/// Returns a tuple containing the public key and the secret key in Base58 encoded format.
pub fn create_wallet() -> (String, String) {
    let keypair = Keypair::new();
    let pubkey_str = keypair.pubkey().to_string();
    let secret_key_bytes = keypair.to_bytes();
    let secret_key_str = bs58::encode(secret_key_bytes).into_string();

    (pubkey_str, secret_key_str)
}

#[pg_extern]
fn pc_create_wallet(
) -> Result<TableIterator<'static, (name!(publik_key, String), name!(secret_key, String))>, String>
{
    let mut results = Vec::new();
    let (public_key, secret_key) = create_wallet();
    results.push((public_key, secret_key));
    Ok(TableIterator::new(results))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_wallet_address() {
        let wallet_address_result = create_wallet_address();
        assert!(
            wallet_address_result.is_ok(),
            "Failed to create wallet address"
        );
    }
}
