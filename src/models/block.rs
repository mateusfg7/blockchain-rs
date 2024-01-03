use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,     // The index which the block is stored
    pub timestamp: u64, // The time that current block was created

    pub proof_of_work: u64,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String) -> Self {
        Block {
            index,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash,
            hash: String::default(),
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut block_data = self.clone();
        block_data.hash = String::default();

        let serialized_block_data = serde_json::to_string(&block_data).unwrap();

        digest(serialized_block_data)
    }
}
