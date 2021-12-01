use crate::header::Header;
use crate::transaction::UnverifiedTransaction;
#[derive(Default,Clone)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<UnverifiedTransaction>,
    pub uncles: Vec<Header>,
}

impl Block {
    pub fn header(&self) -> &Header {
        self.header.clone()
    }
    pub fn hash(&self) -> H256 {
        self.header.hash()
    }
}
