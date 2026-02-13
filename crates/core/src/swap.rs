use serde::{Deserialize, Serialize};
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwapState {
    Created, 
    InvoiceGenerated, 
    OnChainFunded,
    LighteningFunded,
    Lightening
    Pending,
    Completed,
    Failed,
}