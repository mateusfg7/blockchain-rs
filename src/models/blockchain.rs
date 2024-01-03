use super::block::Block;

type Blocks = Vec<Block>;

#[derive(Debug)]
pub struct Blockchain {
    pub genesis_block: Block,
    pub chain: Blocks,
    pub difficulty: usize,
}
