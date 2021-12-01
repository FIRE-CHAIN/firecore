
use crate::bytes::Bytes;
use ethereum_types::{Address, Bloom, H256, U256};

#[derive(Clone)]
pub struct Header {
    /// Parent hash.
    pub parent_hash: H256,
    /// Block timestamp.
    pub timestamp: u64,
    /// Block number.
    pub number: u64,
    /// Block author.
    pub author: Address,
    /// Transactions root.
    pub transactions_root: H256,
    /// Block uncles hash.
    pub uncles_hash: H256,
    /// extra_data
    pub extra_data: Bytes,
    /// State root.
    pub state_root: H256,
    /// Block receipts root.
    pub receipts_root: H256,
    /// Block bloom.
    pub log_bloom: Bloom,
    /// Gas used for contracts execution.
    pub gas_used: U256,
    /// Block gas limit.
    pub gas_limit: U256,
    /// Block difficulty.
    pub difficulty: U256,
    /// Vector of post-RLP-encoded fields.
    pub seal: Vec<Bytes>,
    /// Base fee per gas. Introduced by EIP1559.
    pub base_fee_per_gas: Option<U256>,
    /// Memoized hash of that header and the seal.
    pub hash: Option<H256>,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            parent_hash: H256::default(),
            timestamp: 0,
            number: 0,
            author: Address::default(),

            transactions_root: H256::default(),
            uncles_hash: H256::default(),
            extra_data: vec![],

            state_root: H256::default(),
            receipts_root: H256::default(),
            log_bloom: Bloom::default(),
            gas_used: U256::default(),
            gas_limit: U256::default(),

            difficulty: U256::default(),
            seal: vec![],
            hash: None,
            base_fee_per_gas: None,
        }
    }
}

impl Header {
    pub fn new() -> Self {
        Header::default()
    }

    pub fn set_timestamp(&mut self, a: u64) {
        change_field(&mut self.hash, &mut self.timestamp, a);
    }
}

fn change_field<T>(hash: &mut Option<H256>, field: &mut T, value: T)
where
    T: PartialEq<T>,
{
    if field != &value {
        *field = value;
        *hash = None;
    }
}
