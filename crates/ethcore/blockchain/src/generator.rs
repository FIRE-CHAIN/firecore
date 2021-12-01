use std::collections::VecDeque;

use common_types::{
    block::Block,
    transaction::{SignedTransaction, Transaction},
};
use ethereum_types::{Bloom, U256, H256};

pub struct BlockOptions {
    /// Difficulty
    pub difficulty: U256,
    /// Set bloom filter
    pub bloom: Bloom,
    /// Transactions included in blocks
    pub transactions: Vec<SignedTransaction>,
}

impl Default for BlockOptions {
    fn default() -> Self {
        BlockOptions {
            difficulty: 10.into(),
            bloom: Bloom::default(),
            transactions: Vec::new(),
        }
    }
}

pub struct BlockBuilder {
    blocks: VecDeque<Block>,
}

impl BlockBuilder {
    /// Creates a new block builder with the given genesis block.
    pub fn genesis() -> Self {
        let blocks = VecDeque::with_capacity(1);
        blocks.push_back(Block::default());
        BlockBuilder { blocks }
    }

    pub fn add_block(&self) -> Self {
        self.add_block_with(|| BlockOptions::default())
    }

    pub fn add_block_with<T>(&self, get_metadata: T) -> Self
    where
        T: Fn() -> BlockOptions,
    {
        self.add_blocks_with(1, get_metadata)
    }

    pub fn add_blocks_with<T>(&self, count: usize, get_metada: T) -> Self
    where
        T: Fn() -> BlockOptions,
    {
        let mut parent_hash: H256 = self.last().hash();
        let mut parent_number = self.last().number();
        let mut blocks = VecDeque::with_capacity(count);

        for _ in 0..count {
            let mut block = Block::default();
            let metadata = get_metada();
            let block_number = parent_number + 1;
            let transactions = metadata.transactions;
            let transactions_root = ordered_trie_root(transactions.iter().map(|tx| tx.encode()));

            block.header.set_parent_hash(parent_hash);
            block.header.set_number(block_number);
            block.header.set_log_bloom(metadata.bloom);
            block.header.set_difficulty(metadata.difficulty);
            block.header.set_transactions_root(transactions_root);
            block.transactions = transactions;

            parent_hash = block.hash();
            parent_number = block_number;

            blocks.push_back(block);
        }

        BlockBuilder { blocks }
    }
    
    pub fn last(&self) -> &Block {
        self.blocks.back().expect("No blocks in builder")
    }
}
