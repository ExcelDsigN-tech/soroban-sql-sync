use crate::config::Config;
use anyhow::Result;
use tracing::info;

pub struct EventListener {
    config: Config,
}

impl EventListener {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&self) -> Result<()> {
        info!(
            "EventListener starting for contracts: {:?}",
            self.config.contract_ids
        );
        // Phase 2: implement RPC polling loop here
        Ok(())
    }
}
