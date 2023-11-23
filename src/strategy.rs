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
use ethers::types::{Address, Bytes, Log, H256, U256};

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
        // Trigger a heartbeat with the current last beat time when starting up so the bot doesn't miss one
        let heart = OlympusHeart::new(self.heart, self.provider.clone());
        let last_beat = heart.last_beat().call().await.unwrap();
        let event = Event::HeartBeat(Log {
            address: self.heart,
            topics: vec![H256::from_low_u64_be(last_beat)],
            data: Bytes::new(),
            block_hash: None,
            block_number: None,
            transaction_hash: None,
            transaction_index: None,
            log_index: None,
            transaction_log_index: None,
            log_type: None,
            removed: None,
        });
        self.process_event(event).await;
        Ok(())
    }

    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::HeartBeat(log) => {
                info!("Processing heartbeat event.");
                // Steps
                // 0. Get updated info from contract and current token prices
                let last_beat = U256::from_big_endian(log.topics[0].as_bytes()).as_u64();
                debug!("Last beat: {}", last_beat);
                let heart = OlympusHeart::new(self.heart, self.provider.clone());
                let frequency = heart.frequency().call().await.unwrap();
                debug!("Frequency: {}", frequency);
                let auction_duration = heart.auction_duration().call().await.unwrap();
                debug!("Auction duration: {}", auction_duration);
                let max_reward = heart.max_reward().call().await.unwrap();
                debug!("Max reward: {}", max_reward);

                let eth_price = U256::from(
                    ((match get_token_price(&self.web_client, "ethereum").await {
                        Ok(price) => price,
                        Err(e) => {
                            error!("Error getting ETH price from API: {}", e);
                            return vec![];
                        }
                    }) * (1e6 as f64)) as u64,
                );
                debug!("ETH price: {}", eth_price);

                let ohm_price = U256::from(
                    ((match get_token_price(&self.web_client, "olympus").await {
                        Ok(price) => price,
                        Err(e) => {
                            error!("Error getting OHM price from API: {}", e);
                            return vec![];
                        }
                    }) * (1e6 as f64)) as u64,
                );
                debug!("OHM price: {}", ohm_price);

                let mut tx = heart.beat().tx;

                // 1. Once heartbeat is seen, sleep until the next beat is available
                let sleep_duration =
                    Duration::from_secs(frequency - (get_sys_time_in_secs() - last_beat));
                info!(
                    "Sleeping for {} seconds until next beat is available.",
                    sleep_duration.as_secs()
                );
                sleep(sleep_duration).await;

                // 2. Once the next beat is available, check if it is profitable above our threshold every 12 seconds
                let blocks = auction_duration / 12;
                for b in 0..=blocks {
                    // Get gas estimate for the heartbeat
                    let gas_estimate = self.provider.estimate_gas(&tx, None).await.unwrap();
                    debug!("Gas estimate: {}", gas_estimate);

                    // Get current gas price
                    debug!("Retrieving gas price from provider.");
                    let gas_price = match self.provider.get_gas_price().await {
                        Ok(gas_price) => gas_price,
                        Err(e) => {
                            error!("Error getting gas price from provider: {}", e);
                            return vec![];
                        }
                    };
                    debug!("Gas price: {}", gas_price);

                    // Calculate current gas fee with gas estimate, gas price and eth price
                    let gas_fee = (gas_estimate * gas_price * eth_price) / U256::from(1e6 as u64);
                    debug!("Gas fee (in dollars * 1e18): {}", gas_fee);

                    // Calculate current reward
                    let reward = (max_reward * U256::from(b * 1e9 as u64) * ohm_price)
                        / U256::from(blocks * 1e6 as u64);
                    debug!("Reward (in dollars * 1e18): {}", reward);

                    // If reward is greater than gas fee + profit threshold, then tx is likely to be included
                    if reward
                        > gas_fee
                            + U256::from((self.profit_threshold * 1e6f64) as u64)
                                * U256::from(1e12 as u64)
                    {
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

#[cfg(test)]
mod tests {
    #[test]
    fn reward_math_no_overflows() {
        use ethers::types::U256;
        let eth_price = U256::from(2000e6 as u64);
        let ohm_price = U256::from(11e6 as u64);
        let gas_estimate = U256::from(500000 as u64);
        let gas_price = U256::from(100e9 as u64);
        let max_reward = U256::from(40e9 as u64);
        let auction_duration = 1200;
        let blocks = auction_duration / 12;
        let block = 10u64;

        let gas_fee = (gas_estimate * gas_price * eth_price) / U256::from(1e6 as u64);
        let reward = (max_reward * U256::from(block * 1e9 as u64) * ohm_price)
            / U256::from(blocks * 1e6 as u64);

        let threshold = gas_fee + U256::from((10.0f64 * 1e6f64) as u64) * U256::from(1e12 as u64);

        println!("Gas fee: {}", gas_fee);
        println!("Reward: {}", reward);
        println!("Threshold: {}", threshold);
    }
}
