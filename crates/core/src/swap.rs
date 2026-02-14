use serde::{Deserialize, Serialize};
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwapState {
    Created, 
    InvoiceGenerated, 
    OnChainFunded,
    LighteningFunded,
    LighteningSettled,
    PreimageRevealed,
    Completed,
    Cancelled,
    Refunded,
    Refunfdable,

}

#[derive(Error, Debug)]
pub enum SwapError {
    #[error("invalid state transition")]
    InvalidTransition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swap {
    pub id: Uuid,
    pub payment_hash: String,
    pub amount_sat: u64,
    pub state: SwapState,
}