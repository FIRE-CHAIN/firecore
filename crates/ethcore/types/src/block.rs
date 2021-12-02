use crate::header::Header;
use crate::transaction::UnverifiedTransaction;
#[derive(Default)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<UnverifiedTransaction>,
    pub uncles: Vec<Header>,
}

impl Block {}
