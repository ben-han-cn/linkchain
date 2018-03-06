use ethereum_types::{H256, U256, Address, Bloom};
use bytes::Bytes;
use rlp::{UntrustedRlp, RlpStream, Encodable, Decodable, DecoderError};
use keccak_hash::{KECCAK_NULL_RLP, keccak};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub nonce: u64,
    pub gas_price: U256,
    pub gas_limit: U256,
    pub recipient: Option<Address>,
    pub amount: U256,
    pub payload: Bytes,
    pub v: U256,
    pub r: U256,
    pub s: U256,
}

impl Transaction {
    pub fn new() -> Self {
    }
}


impl Encodable for Transaction {
    pub fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(9);
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas_limit);
        match self.recipient {
            Some(ref address) => s.append(address),
                None => s.append_internal(&""),
        };
        s.append(&self.amount);
        s.append(&self.payload);
        s.append(&self.v);
        s.append(&self.r);
        s.append(&self.s);
    }
}

impl Decodable for Transaction {
    fn decode(d: &UntrustedRlp) -> Result<Self, DecoderError> {
        if d.item_count()? != 9 {
            return Err(DecoderError::RlpIncorrectListLen);
        }
        Ok(Transaction {
            nonce: r.val_at(0)?,
            gas_price: r.val_at(1)?,
            gas_limit: r.val_at(2)?,
            recipient: r.val_at(3)?,
            amount: r.val_at(4)?,
            payload: r.val_at(5)?,
            v: r.val_at(6)?,
            r: r.val_at(7)?,
            s: r.val_at(8)?,
        })
    }
}


#[cfg(test)]
mod tests {
    /*
       use rustc_hex::FromHex;
       use rlp;
       use super::Header;
       */

    #[test]
    fn test_header_seal_fields() {}

    #[test]
    fn decode_and_encode_header() {}
}
