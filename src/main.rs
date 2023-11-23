// pacemaker/main - service that calls the Olympus V3 heart beat at a target profit threshold

mod bindings;
mod executor;
mod strategy;
mod types;
mod utils;

use anyhow::Result;
use std::sync::Arc;

use artemis_core::{
    collectors::log_collector::LogCollector,
    engine::Engine,
    types::{CollectorMap, ExecutorMap},
};
use clap::Parser;
use dotenvy::dotenv;
use ethers::{
    middleware::MiddlewareBuilder,
    providers::{Middleware, Provider, Ws},
    signers::{LocalWallet, Signer},
    types::{Address, Filter, ValueOrArray},
};

use executor::ProtectExecutor;
use strategy::PacemakerStrategy;
use types::{Action, Event};

use tracing::info;
use tracing_subscriber::EnvFilter;

/// CLI Options.
#[derive(Parser, Debug)]
pub struct Args {
    /// Ethereum node WS endpoint. Used for reading blockchain data.
    #[arg(long, env = "WS_RPC_URL")]
    pub ws_rpc_url: String,
    /// Ethereum node HTTP endpoint. Used for sending transactions.
    /// Make sure to use a MEV protected RPC (e.g. Flashbots) to avoid frontrunning on chains where that is an issue.
    #[arg(long, env = "HTTP_RPC_URL")]
    pub http_rpc_url: String,
    /// Private key for sending txs.
    #[arg(long, env = "PRIVATE_KEY")]
    pub private_key: String,
    /// Address of the Olympus V3 Heart contract
    #[arg(long, env = "HEART_ADDRESS")]
    pub heart_address: Address,
    /// Profit threshold for calling the heart beat (in $)
    #[arg(long, env = "PROFIT_THRESHOLD", default_value = "10.0")]
    pub profit_threshold: f64,
}

/// One instance of this service needs to be run for each combination of chain_id, aggregator, and settlement contract.
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Parse CLI args, using ENV vars if not provided
    let args = Args::parse();

    // Set up tracing and parse args.
    let env_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::fmt()
        .with_env_filter(env_layer)
        .with_target(true)
        .init();

    //  Set up providers and signers.
    let read_provider = Provider::<Ws>::connect_with_reconnects(&args.ws_rpc_url, 100)
        .await
        .unwrap();
    let write_provider = Provider::try_from(&args.http_rpc_url).unwrap();

    // Confirm that the input chain_id and chain_id of provider match
    let read_provider_chain_id = read_provider.get_chainid().await.unwrap().as_u64();
    let write_provider_chain_id = write_provider.get_chainid().await.unwrap().as_u64();
    if read_provider_chain_id != write_provider_chain_id {
        panic!(
            "Chain ID of read provider ({}) does not match write provider ({})",
            read_provider_chain_id, write_provider_chain_id
        );
    }

    info!(
        "Starting pacemaker for Heart contract {} on chain {}",
        args.heart_address, write_provider_chain_id
    );

    // Parse key for signer and add to write provider
    let wallet: LocalWallet = args.private_key.parse().unwrap();
    let wallet = wallet.with_chain_id(write_provider_chain_id);
    let address = wallet.address();

    let write_provider = Arc::new(
        write_provider
            .nonce_manager(address)
            .with_signer(wallet.clone()),
    );

    // Set up engine.
    let mut engine: Engine<Event, Action> = Engine::default();

    // Set up collectors
    // Create collector using event filters
    let beat_filter = Filter::new()
        .from_block(read_provider.get_block_number().await.unwrap().as_u64())
        .event("Beat(uint256)")
        .address(ValueOrArray::Value(args.heart_address));
    let beat_collector = Box::new(LogCollector::new(
        Arc::new(read_provider.clone()),
        beat_filter,
    ));
    let beat_collector = CollectorMap::new(beat_collector, Event::HeartBeat);
    engine.add_collector(Box::new(beat_collector));

    // Set up strategy.
    let strategy = PacemakerStrategy::new(
        Arc::new(read_provider.clone()),
        &args.heart_address,
        &address,
        &args.profit_threshold,
    );
    engine.add_strategy(Box::new(strategy));

    // Set up executor.
    // Note: make sure to use the Flashbots RPC on mainnet to avoid MEV
    let rpc_executor = Box::new(ProtectExecutor::new(write_provider.clone()));
    let rpc_executor = ExecutorMap::new(rpc_executor, |action| match action {
        Action::SubmitTx(txn) => Some(txn),
    });
    engine.add_executor(Box::new(rpc_executor));

    // Start engine.
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            if res.is_err() {
                return Result::Err(anyhow::Error::msg(res.err().unwrap()));
            } else {
                info!("res: {:?}", res);
            }
        }
    }

    Ok(())
}
