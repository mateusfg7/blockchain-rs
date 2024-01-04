use chrono::Utc;

use super::block::Block;

type Blocks = Vec<Block>;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub genesis_block: Block,
    pub chain: Blocks,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        // First block in the chain
        let mut genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default(),
        };

        genesis_block.hash = genesis_block.calculate_hash();

        let chain: Blocks = vec![genesis_block.clone()];

        Blockchain {
            genesis_block,
            chain,
            difficulty,
        }
    }

    pub fn add_block(&mut self) {
        let previous_hash = self.chain[self.chain.len() - 1].hash.clone();
        let mut new_block = Block::new(self.chain.len() as u64, previous_hash);

        new_block.mine(self.clone());

        self.chain.push(new_block.clone());

        println!("Index ----------- {}", new_block.index);
        println!("Timestamp ------- {}", new_block.timestamp);
        println!("Proof of work --- {}", new_block.proof_of_work);
        println!("Previous hash --- {}", new_block.previous_hash);
        println!("Hash ------------ {}", new_block.hash);

        println!("");
    }
}
