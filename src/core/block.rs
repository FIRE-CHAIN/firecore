use super::Transaction;
use blake2::{Blake2b, Digest};

#[derive(Debug)]
pub struct Block {
    pub index: u128,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u128,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        index: u128,
        prev_hash: String,
        nonce: u128,
        transactions: Vec<Transaction>,
    ) -> Self {
        Block {
            index,
            prev_hash,
            hash: String::new(),
            nonce,
            timestamp: 0,
            transactions,
        }
    }
    pub fn calculate_hash(&self) -> Vec<u8> {
        let mut hasher = Blake2b::new();

        // write input message
        let block_as_string = format!("{:?}", (&self.prev_hash, &self.nonce));
        hasher.update(&block_as_string);

        return Vec::from(hasher.finalize().as_ref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_hash() {
        let block = Block::new(0, "111".to_string(), 11, Vec::new());
        let hash = block.calculate_hash();

        println!("has:{:?}", hash);
        assert_eq!(hash.len(), 64);
    }
}
