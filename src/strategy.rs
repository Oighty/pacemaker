use anyhow::Result;
use async_trait::async_trait;

use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info};

// External clients
use ethers::providers::Middleware;
use reqwest::Client;

// Artemis types
use artemis_core::types::Strategy;

// Ethereum types
use ethers::types::{Address, U256};

// Local types
use crate::bindings::olympus_heart::OlympusHeart;
use crate::types::{Action, Event};
use crate::utils::{get_sys_time_in_secs, get_token_price};


#[derive(Debug, Clone)]
pub struct PacemakerStrategy<M> {
    /// Provider for reading data from the blockchain.
    pub provider: Arc<M>,
    /// Address of the Olympus Heart contract
    pub heart: Address,
    /// Address of the executor.
    pub executor: Address,
    /// Profit threshold, in $
    pub profit_threshold: f64,
    /// HTTP client for making API requests.
    pub web_client: Client,
}

impl<M: Middleware + 'static> PacemakerStrategy<M> {
    pub fn new(
        provider: Arc<M>,
        heart: &Address,
        executor: &Address,
        profit_threshold: &f64,
    ) -> Self {
        Self {
            provider: provider.clone(),
            heart: *heart,
            executor: *executor,
            profit_threshold: *profit_threshold,
            web_client: Client::new(),
        }
    }
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for PacemakerStrategy<M> {
    async fn sync_state(&mut self) -> Result<()> {
        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::HeartBeat(log) => {
                // Steps
                // 0. Get updated info from contract and current token prices
                let last_beat = U256::from_big_endian(log.topics[0].as_bytes()).as_u64();
                let heart = OlympusHeart::new(self.heart, self.provider.clone());
                let frequency = heart.frequency().call().await.unwrap();
                let auction_duration = heart.auction_duration().call().await.unwrap();
                let max_reward = heart.max_reward().call().await.unwrap();

                let eth_price = match get_token_price(&self.web_client, "ethereum").await {
                    Ok(price) => price,
                    Err(e) => {
                        error!("Error getting ETH price from API: {}", e);
                        return vec![];
                    }
                };

                let ohm_price = match get_token_price(&self.web_client, "olympus").await {
                    Ok(price) => price,
                    Err(e) => {
                        error!("Error getting OHM price from API: {}", e);
                        return vec![];
                    }
                };
                
                let mut tx = heart.beat().tx;

                // 1. Once heartbeat is seen, sleep until the next beat is available
                let sleep_duration = Duration::from_secs(frequency - (get_sys_time_in_secs() - last_beat));
                info!("Sleeping for {} seconds until next beat is available.", sleep_duration.as_secs());
                sleep(sleep_duration).await;

                // 2. Once the next beat is available, check if it is profitable above our threshold every 12 seconds
                let blocks = auction_duration / 12;
                for b in 0..=blocks {
                    // Get gas estimate for the heartbeat
                    let gas_estimate = self.provider.estimate_gas(&tx, None).await.unwrap();
                    
                    // Get current gas price
                    debug!("Retrieving gas price from provider.");
                    let gas_price = match self.provider.get_gas_price().await {
                        Ok(gas_price) => gas_price,
                        Err(e) => {
                            error!("Error getting gas price from provider: {}", e);
                            return vec![];
                        }
                    };

                    // Calculate current gas fee with gas estimate, gas price and eth price
                    let gas_fee = (((gas_estimate * gas_price).as_u64() as f64) * eth_price) / 1e18f64;

                    // Calculate current reward
                    let reward = ((((max_reward.as_u64() * b) / blocks) as f64) * ohm_price) / 1e9f64;

                    // If reward is greater than gas fee + profit threshold, then tx is likely to be included
                    if reward > gas_fee + self.profit_threshold {
                        info!("Heartbeat is profitable. Submitting transaction.");
                        
                        // Set gas price to the current
                        tx.set_gas_price(gas_price);

                        return vec![Action::SubmitTx(tx)];
                    }

                    // Sleep for 12 seconds
                    sleep(Duration::from_secs(12)).await;
                }

                return vec![];
            }
        }
    }
}
