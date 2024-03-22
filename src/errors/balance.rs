use std::{error::Error, fmt};

#[derive(Debug)]
pub enum BalanceError {
    InvalidAddress(String),
    RpcError(String),
}

impl fmt::Display for BalanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BalanceError::InvalidAddress(msg) => write!(f, "Invalid address: {}", msg),
            BalanceError::RpcError(msg) => write!(f, "RPC error: {}", msg),
        }
    }
}

impl Error for BalanceError {}
