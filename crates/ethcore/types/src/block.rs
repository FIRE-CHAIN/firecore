use crate::header::Header;
use crate::transaction::UnverifiedTransaction;


pub struct Block{
    pub header: Header,
    pub transactions: Vec<UnverifiedTransaction>,
    pub uncles: Vec<Header>,
}