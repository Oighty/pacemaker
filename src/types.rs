use ethers::types::{Log, transaction::eip2718::TypedTransaction};

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
