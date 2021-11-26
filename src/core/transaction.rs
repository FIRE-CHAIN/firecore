pub struct Transaction {
    pub nonce: u32,
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    created_at: SystemTime,
    signature: String,
}
