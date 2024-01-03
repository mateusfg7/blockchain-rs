use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,     // The index which the block is stored
    pub timestamp: u64, // The time that current block was created

    pub proof_of_work: u64,
    pub previous_hash: String,
    pub hash: String,
}
