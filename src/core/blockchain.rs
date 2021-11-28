use super::{Block, Transaction};

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: Vec::new(),
            pending_transactions: Vec::new(),
        }
    }

    // pub fn append_block(&mut self, block: Block) -> Result<(), string> {
    //     self.blocks.push(block);
    //     Some("ok")
    // }
}
