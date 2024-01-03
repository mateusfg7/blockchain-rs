use chrono::Utc;

use super::block::Block;

type Blocks = Vec<Block>;

#[derive(Debug)]
pub struct Blockchain {
    pub genesis_block: Block,
    pub chain: Blocks,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        // First block in the chain
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default(),
        };

        let chain: Blocks = vec![genesis_block.clone()];

        Blockchain {
            genesis_block,
            chain,
            difficulty,
        }
    }
}
