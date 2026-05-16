use anyhow::{bail, Context, Result};
use std::env;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub soroban_rpc_url: String,
    pub contract_ids: Vec<String>,
    pub api_addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL environment variable not set"))?;

        let soroban_rpc_url = env::var("SOROBAN_RPC_URL")
            .map_err(|_| anyhow::anyhow!("SOROBAN_RPC_URL environment variable not set"))?;

        let contract_ids_str = env::var("CONTRACT_IDS")
            .map_err(|_| anyhow::anyhow!("CONTRACT_IDS environment variable not set"))?;

        let contract_ids: Vec<String> = contract_ids_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if contract_ids.is_empty() {
            bail!("CONTRACT_IDS must contain at least one contract ID");
        }

        let api_addr = env::var("API_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:3000".to_string())
            .parse()
            .context("API_ADDR must be a valid socket address, e.g. 127.0.0.1:3000")?;

        Ok(Config {
            database_url,
            soroban_rpc_url,
            contract_ids,
            api_addr,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env() {
        env::set_var("DATABASE_URL", "postgres://localhost/test");
        env::set_var("SOROBAN_RPC_URL", "https://soroban-rpc.stellar.org");
        env::set_var("CONTRACT_IDS", "CA123,CA456");

        let config = Config::from_env();
        assert!(config.is_ok());

        let cfg = config.unwrap();
        assert_eq!(cfg.contract_ids.len(), 2);
        assert_eq!(cfg.api_addr.to_string(), "127.0.0.1:3000");
    }
}
