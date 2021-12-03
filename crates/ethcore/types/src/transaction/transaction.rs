use crate::{
    crypto::publickey::{recover, Secret, Signature},
    hash::keccak,
};
use ethereum_types::{Address, BigEndianHash, Public, H256, U256};
use parity_crypto::publickey::{self, public_to_address};
use rlp::{DecoderError, Rlp, RlpStream};

type Bytes = Vec<u8>;

#[derive(Clone,Debug)]
pub enum Action {
    Create,
    Call(Address),
}

impl rlp::Decodable for Action {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        if rlp.is_empty() {
            if rlp.is_data() {
                Ok(Action::Create)
            } else {
                Err(DecoderError::RlpExpectedToBeData)
            }
        } else {
            Ok(Action::Call(rlp.as_val()?))
        }
    }
}

impl rlp::Encodable for Action {
    fn rlp_append(&self, s: &mut RlpStream) {
        match *self {
            Action::Create => s.append_internal(&""),
            Action::Call(ref addr) => s.append_internal(addr),
        };
    }
}

#[derive(Clone,Debug)]
pub enum TypedTransaction {
    Legacy(Transaction),
}

#[derive(Clone,Debug)]
pub struct Transaction {
    pub nonce: U256,
    pub gas_price: U256,
    pub gas: U256,
    pub action: Action,
    pub value: U256,
    pub data: Bytes,
}

#[derive(Clone,Debug)]
pub struct SignatureComponents {
    pub standard_v: u8,
    pub r: U256,
    pub s: U256,
}

impl SignatureComponents {
    pub fn rlp_append(&self, s: &mut RlpStream) {
        s.append(&self.standard_v);
        s.append(&self.r);
        s.append(&self.s);
    }

    pub fn rlp_append_with_chain_id(&self, s: &mut RlpStream, chain_id: Option<u64>) {
        s.append(&signature::add_chain_replay_protection(
            self.standard_v,
            chain_id,
        ));

        s.append(&self.r);
        s.append(&self.s);
    }
}

pub mod signature {
    pub fn add_chain_replay_protection(v: u8, chain_id: Option<u64>) -> u64 {
        v as u64
            + if let Some(n) = chain_id {
                35 + n * 2
            } else {
                27
            }
    }
}
#[derive(Clone,Debug)]
pub struct UnverifiedTransaction {
    pub unsigned: TypedTransaction,
    pub signature: SignatureComponents,
    pub chain_id: Option<u64>,
    pub hash: H256,
}

#[derive(Clone,Debug)]
pub struct SignedTransaction {
    #[allow(dead_code)]
    transaction: UnverifiedTransaction,
    #[allow(dead_code)]
    sender: Address,
    #[allow(dead_code)]
    public: Option<Public>,
}

impl SignedTransaction {
    pub fn new(transaction: UnverifiedTransaction) -> Result<Self, publickey::Error> {
        if transaction.is_unsigned() {
            return Err(publickey::Error::InvalidSignature);
        }

        let public = transaction.recover_public()?;
        let sender = public_to_address(&public);
        Ok(SignedTransaction {
            transaction,
            sender,
            public: Some(public),
        })
    }
}

impl Transaction {
    fn encode(&self, chain_id: Option<u64>, signature: Option<&SignatureComponents>) -> Vec<u8> {
        let mut stream = RlpStream::new();
        self.encode_rlp(&mut stream, chain_id, signature);
        stream.out().to_vec()
    }

    fn encode_rlp(
        &self,
        rlp: &mut RlpStream,
        chain_id: Option<u64>,
        signature: Option<&SignatureComponents>,
    ) {
        let list_size = if chain_id.is_some() || signature.is_some() {
            9
        } else {
            6
        };

        rlp.begin_list(list_size);

        self.rlp_append_data_open(rlp);

        if let Some(signature) = signature {
            signature.rlp_append_with_chain_id(rlp, chain_id);
        } else {
            if let Some(n) = chain_id {
                rlp.append(&n);
                rlp.append(&0u8);
                rlp.append(&0u8);
            }
        }
    }

    fn rlp_append_data_open(&self, s: &mut RlpStream) {
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        s.append(&self.action);
        s.append(&self.value);
        s.append(&self.data);
    }
}

impl TypedTransaction {
    pub fn signature_hash(&self, chain_id: Option<u64>) -> H256 {
        keccak(match self {
            Self::Legacy(tx) => tx.encode(chain_id, None),
        })
    }

    fn encode(&self, chain_id: Option<u64>, signature: &SignatureComponents) -> Vec<u8> {
        let signature = Some(signature);
        match self {
            Self::Legacy(tx) => tx.encode(chain_id, signature),
        }
    }

    pub fn sign(self, secret: &Secret, chain_id: Option<u64>) -> SignedTransaction {
        let sig = publickey::sign(secret, &self.signature_hash(chain_id))
            .expect("data is valid and context has signing capabilities; qed");

        SignedTransaction::new(self.with_signature(sig, chain_id))
            .expect("secret is valid so it's recoverable")
    }

    pub fn with_signature(self, sig: Signature, chain_id: Option<u64>) -> UnverifiedTransaction {
        UnverifiedTransaction {
            unsigned: self,
            chain_id,
            signature: SignatureComponents {
                r: sig.r().into(),
                s: sig.s().into(),
                standard_v: sig.v().into(),
            },
            hash: H256::zero(),
        }
        .compute_hash()
    }
}

impl UnverifiedTransaction {
    pub fn is_unsigned(&self) -> bool {
        self.signature.r.is_zero() && self.signature.s.is_zero()
    }

    pub fn standard_v(&self) -> u8 {
        self.signature.standard_v
    }

    pub fn signature(&self) -> Signature {
        let r = BigEndianHash::from_uint(&self.signature.r);
        let s = BigEndianHash::from_uint(&self.signature.s);

        Signature::from_rsv(&r, &s, self.standard_v())
    }

    pub fn chain_id(&self) -> Option<u64> {
        self.chain_id
    }

    pub fn recover_public(&self) -> Result<Public, publickey::Error> {
        Ok(recover(
            &self.signature(),
            &self.unsigned.signature_hash(self.chain_id()),
        )?)
    }

    pub fn encode(&self) -> Vec<u8> {
        self.unsigned.encode(self.chain_id, &self.signature)
    }

    pub fn compute_hash(mut self) -> UnverifiedTransaction {
        let hash = keccak(&*self.encode());
        self.hash = hash;
        self
    }
}
