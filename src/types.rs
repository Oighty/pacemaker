use ethers::types::{transaction::eip2718::TypedTransaction, Log};

/// Core Event enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Event {
    HeartBeat(Log),
}

/// Core Action enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Action {
    SubmitTx(TypedTransaction),
}
