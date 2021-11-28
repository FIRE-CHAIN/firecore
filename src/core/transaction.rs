use std::time::SystemTime;

#[derive(Debug)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u128,
    pub fee: u128,
    pub nonce: u128,
    created_at: SystemTime,
    signature: String,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u128, nonce: u128) -> Self {
        Transaction {
            sender,
            nonce,
            recipient,
            fee: 0,
            amount,
            created_at: SystemTime::now(),
            signature: String::new(),
        }
    }

    
}
