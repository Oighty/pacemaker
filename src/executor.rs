use std::{sync::Arc, time::Duration};

use crate::utils::{bytes_to_string, get_sys_time_in_secs};
use anyhow::Result;
use artemis_core::types::Executor;
use async_trait::async_trait;
use ethers::{
    middleware::SignerMiddleware,
    providers::Middleware,
    signers::Signer,
    types::{transaction::eip2718::TypedTransaction, H256, U256},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};

/// An executor that sends transactions to the mempool.
pub struct ProtectExecutor<M, S> {
    /// Client for sending transactions and reading data from the blockchain.
    pub provider: Arc<SignerMiddleware<M, S>>,
    /// HTTP client for making API requests.
    pub web_client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[allow(non_snake_case)]
pub struct FlashbotsStatus {
    pub status: String,
    pub hash: H256,
    pub maxBlockNumber: u64,
    pub fastMode: bool,
    pub seenInMempool: bool,
}

impl<M: Middleware, S: Signer> ProtectExecutor<M, S> {
    pub fn new(provider: Arc<SignerMiddleware<M, S>>) -> Self {
        Self {
            provider,
            web_client: Client::new(),
        }
    }
}

#[async_trait]
impl<M, S> Executor<TypedTransaction> for ProtectExecutor<M, S>
where
    M: Middleware + 'static,
    S: Signer + 'static,
    M::Error: 'static,
{
    /// Send a transaction to the mempool.
    async fn execute(&self, tx: TypedTransaction) -> Result<()> {
        info!("Action received. Submitting to blockchain provider.");

        // Get chain ID from provider
        let chain_id = self.provider.get_chainid().await?;

        // Send the transaction to the RPC
        let pending_txn = self.provider.send_transaction(tx, None).await;

        // If mainnet, we're using flashbots, so we need to check status manually
        // Otherwise, await transaction receipt
        if chain_id == U256::from(1) {
            match pending_txn {
                Ok(result) => {
                    let tx_hash = result.tx_hash();
                    // Ping status from flashbots status API, allow 26 blocks worth of time (the RPC accepts it for 25 blocks, but we give some cushion)
                    let timeout = get_sys_time_in_secs() + (26 * 12) as u64;
                    let mut settled = false;
                    while !settled && get_sys_time_in_secs() < timeout {
                        match self
                            .web_client
                            .get(format!(
                                "https://protect.flashbots.net/tx/{}",
                                bytes_to_string(tx_hash.as_bytes())
                            ))
                            .send()
                            .await
                        {
                            Ok(response) => {
                                match response.json::<FlashbotsStatus>().await {
                                    Ok(resp) => {
                                        debug!("Checking transaction status on relay: {:?}", resp);
                                        if resp.status == "INCLUDED" {
                                            settled = true;
                                            info!("Transaction successful.");
                                        } else if resp.status == "FAILED"
                                            || resp.status == "CANCELLED"
                                        {
                                            settled = true;
                                            warn!("Transaction failed or cancelled.");
                                        }
                                    }
                                    Err(e) => {
                                        warn!("Failed to parse Flashbots response: {}", e);
                                    }
                                };
                            }
                            Err(e) => {
                                // Fail silently here and try again
                                warn!("Flashbots status check failed: {}", e);
                            }
                        }
                        tokio::time::sleep(Duration::from_secs(12)).await;
                    }
                    if !settled {
                        warn!("Transaction was not settled before timeout.")
                    }
                }
                Err(e) => {
                    error!("Error sending transaction to flashbots. {}", e);
                }
            }
        } else {
            match pending_txn {
                Ok(pending_txn) => {
                    info!(
                        "Transaction sent: {}. Waiting on receipt.",
                        pending_txn.to_string()
                    );
                    match pending_txn.await {
                        Ok(Some(receipt)) => {
                            info!(
                                "Transaction mined: {}",
                                receipt.transaction_hash.to_string()
                            );
                        }
                        Ok(None) => {
                            error!("No receipt returned.");
                        }
                        Err(e) => {
                            error!("Error waiting for transaction receipt: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Error sending transaction to mempool: {}", e);
                }
            }
        };

        Ok(())
    }
}
