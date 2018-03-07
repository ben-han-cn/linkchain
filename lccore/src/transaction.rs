use ethereum_types::{Address, Bloom, H256, U256};
use bytes::Bytes;
use rlp::{Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Recipient(Option<Address>);

impl Encodable for Recipient {
    fn rlp_append(&self, s: &mut RlpStream) {
        match self.0 {
            Some(ref addr) => s.append_internal(addr),
            None => s.append_internal(&""),
        };
    }
}

impl Decodable for Recipient {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        if rlp.is_empty() {
            Ok(Recipient(None))
        } else {
            rlp.as_val::<Address>().map(|addr| Recipient(Some(addr)))
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub nonce: u64,
    pub gas_price: U256,
    pub gas_limit: U256,
    pub recipient: Recipient,
    pub amount: U256,
    pub payload: Bytes,
    pub v: U256,
    pub r: U256,
    pub s: U256,
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            nonce: 0,
            gas_price: U256::default(),
            gas_limit: U256::default(),
            recipient: Recipient(None),
            amount: U256::default(),
            payload: vec![],
            v: U256::default(),
            r: U256::default(),
            s: U256::default(),
        }
    }
}

impl Encodable for Transaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(9);
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas_limit);
        s.append(&self.recipient);
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
            nonce: d.val_at(0)?,
            gas_price: d.val_at(1)?,
            gas_limit: d.val_at(2)?,
            recipient: d.val_at(3)?,
            amount: d.val_at(4)?,
            payload: d.val_at(5)?,
            v: d.val_at(6)?,
            r: d.val_at(7)?,
            s: d.val_at(8)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use rustc_hex::FromHex;
    use rlp;
    use super::{Recipient, Transaction};
    use ethereum_types::{Address, Bloom, U256};

    #[test]
    fn test_tx_decode() {
        let tx_rlp = "f86103018207d094b94f5374fce5edbc8e2a8697c15331677e6ebf0b0a8255441ca098ff921201554726367d2be8c804a7ff89ccf285ebc57dff8ae4c44b9c19ac4aa08887321be575c8095f789dd4c743dfe42c1820f9231f98a962b210e3ac2452a3".from_hex().unwrap();
        let tx: Transaction = rlp::decode(&tx_rlp);
        assert_eq!(tx.nonce, 3);
        assert_eq!(tx.gas_price, U256::from(1));
        assert_eq!(tx.gas_limit, U256::from(2000));
        assert_eq!(
            tx.recipient,
            Recipient(Some(Address::from(
                "b94f5374fce5edbc8e2a8697c15331677e6ebf0b"
                    .from_hex()
                    .unwrap()
                    .as_slice()
            )))
        );
        assert_eq!(tx.amount, U256::from(10));
        assert_eq!(tx.payload, "5544".from_hex().unwrap().as_slice());
        assert_eq!(
            tx.r,
            U256::from(
                "98ff921201554726367d2be8c804a7ff89ccf285ebc57dff8ae4c44b9c19ac4a"
                    .from_hex()
                    .unwrap()
                    .as_slice()
            )
        );
        assert_eq!(
            tx.s,
            U256::from(
                "8887321be575c8095f789dd4c743dfe42c1820f9231f98a962b210e3ac2452a3"
                    .from_hex()
                    .unwrap()
                    .as_slice()
            )
        );
        assert_eq!(tx.v, U256::from(28));
    }
}
