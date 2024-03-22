pub enum Network {
    Mainnet,
    Devnet,
    Testnet,
    Localhost,
}

impl Network {
    fn default() -> Self {
        Network::Devnet
    }

    pub fn from_str(network_str: &str) -> Self {
        match network_str {
            "mainnet" => Network::Mainnet,
            "devnet" => Network::Devnet,
            "testnet" => Network::Testnet,
            "localhost" => Network::Localhost,
            _ => Network::default(),
        }
    }

    pub fn get_rpc_url(&self) -> String {
        match self {
            Network::Mainnet => "https://api.mainnet-beta.solana.com".to_string(),
            Network::Devnet => "https://api.devnet.solana.com".to_string(),
            Network::Testnet => "https://api.testnet.solana.com".to_string(),
            Network::Localhost => "http://localhost:8899".to_string(),
        }
    }
}
