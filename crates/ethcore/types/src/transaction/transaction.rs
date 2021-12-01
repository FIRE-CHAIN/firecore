use ethereum_types::{Address, H256, Public, U256};

type Bytes = Vec<u8>;

pub enum Action {
    Create,
    Call(Address),
    Create2(Address, U256, Vec<u8>),
}

pub enum TypedTransaction {
    Legacy(Transaction),
}

pub struct Transaction {
    pub nonce: U256,
    pub gas_price: U256,
    pub gas: U256,
    pub action: Action,
    pub value: U256,
    pub data: Bytes,
}

pub struct SignatureComponents {
    pub v: u8,
    pub r: U256,
    pub s: U256,
}

pub struct UnverifiedTransaction {
    pub unsigned: TypedTransaction,
    pub signature: SignatureComponents,
    pub chain_id: Option<u64>,
    pub hash: H256,
}

pub struct SignedTransaction {
    transaction: UnverifiedTransaction,
    sender: Address,
    public: Option<Public>,
}
