use ethereum_types::{H256, U256, Address, Bloom};
use bytes::Bytes;
use rlp::{UntrustedRlp, RlpStream, Encodable, Decodable, DecoderError};
use keccak_hash::{KECCAK_NULL_RLP, keccak};

#[derive(Debug, Clone, Eq)]
pub struct Header {
    pub parent_hash: H256,
    pub coinbase: Address,
    pub state_root: H256,
    pub receipts_root: H256,
    pub transactions_root: H256,
    pub log_bloom: Bloom,
    pub difficulty: U256,
    pub number: u64,
    pub gas_used: U256,
    pub gas_limit: U256,
    pub timestamp: u64,
    pub extra_data: Bytes,
    pub mix_digest: H256,
    pub nonce: [u8;8],
}

impl PartialEq for Header {
    fn eq(&self, c: &Header) -> bool {
        self.parent_hash == c.parent_hash && self.coinbase == c.coinbase &&
            self.state_root == c.state_root && self.receipts_root == c.receipts_root &&
            self.transactions_root == c.transactions_root &&
            self.log_bloom == c.log_bloom && self.difficulty == c.difficulty &&
            self.number == c.number && self.gas_used == c.gas_used &&
            self.gas_limit == c.gas_limit &&
            self.timestamp == c.timestamp &&
            self.extra_data == c.extra_data &&
            self.mix_digest == c.mix_digest &&
            self.nonce == c.nonce
    }
}

impl Default for Header {
    fn default() -> Self {
        Header {
            parent_hash: H256::default(),
            coinbase: Address::default(),
            state_root: KECCAK_NULL_RLP,
            receipts_root: KECCAK_NULL_RLP,
            transactions_root: KECCAK_NULL_RLP,
            log_bloom: Bloom::default(),
            difficulty: U256::default(),
            number: 0,
            gas_used: U256::default(),
            gas_limit: U256::default(),
            timestamp: 0,
            extra_data: vec![],
            mix_digest: H256::default(),
            nonce: [0; 8],
        }
    }
}

impl Header {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parent_hash(&self) -> &H256 {
        &self.parent_hash
    }
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn number(&self) -> u64 {
        self.number
    }
    pub fn coinbase(&self) -> &Address {
        &self.coinbase
    }
    pub fn extra_data(&self) -> &Bytes {
        &self.extra_data
    }
    pub fn state_root(&self) -> &H256 {
        &self.state_root
    }
    pub fn receipts_root(&self) -> &H256 {
        &self.receipts_root
    }
    pub fn transactions_root(&self) -> &H256 {
        &self.transactions_root
    }
    pub fn log_bloom(&self) -> &Bloom {
        &self.log_bloom
    }
    pub fn gas_used(&self) -> &U256 {
        &self.gas_used
    }
    pub fn gas_limit(&self) -> &U256 {
        &self.gas_limit
    }
    pub fn difficulty(&self) -> &U256 {
        &self.difficulty
    }
    pub fn nonce(&self) -> &[u8]{
        &self.nonce
    }

    /// Place this header into an RLP stream `s`, optionally `with_seal`.
    pub fn stream_rlp(&self, s: &mut RlpStream, with_seal: bool) {
        s.begin_list(
            13 +
            if with_seal {
                2
            } else {
                0
            },
            );
        s.append(&self.parent_hash);
        s.append(&self.coinbase);
        s.append(&self.state_root);
        s.append(&self.transactions_root);
        s.append(&self.receipts_root);
        s.append(&self.log_bloom);
        s.append(&self.difficulty);
        s.append(&self.number);
        s.append(&self.gas_limit);
        s.append(&self.gas_used);
        s.append(&self.timestamp);
        s.append(&self.extra_data);
        if with_seal {
            s.append(&self.mix_digest);
            s.append(&&self.nonce[0..8]);
        }
    }

    pub fn rlp(&self, with_seal: bool) -> Bytes {
        let mut s = RlpStream::new();
        self.stream_rlp(&mut s, with_seal);
        s.out()
    }

    pub fn rlp_keccak(&self, with_seal: bool) -> H256 {
        keccak(self.rlp(with_seal))
    }
}

impl Decodable for Header {
    fn decode(r: &UntrustedRlp) -> Result<Self, DecoderError> {
        let mut header = Header {
            parent_hash: r.val_at(0)?,
            coinbase: r.val_at(2)?,
            state_root: r.val_at(3)?,
            transactions_root: r.val_at(4)?,
            receipts_root: r.val_at(5)?,
            log_bloom: r.val_at(6)?,
            difficulty: r.val_at(7)?,
            number: r.val_at(8)?,
            gas_limit: r.val_at(9)?,
            gas_used: r.val_at(10)?,
            timestamp: r.val_at::<U256>(11)?.as_u64(),
            extra_data: r.val_at(12)?,
            mix_digest: H256::default(),
            nonce: [0; 8],
        };

        if r.item_count()? == 15 {
            header.mix_digest = r.val_at(13)?;
            header.nonce.copy_from_slice(&r.at(14)?.as_raw()[0..8]);
        }
        Ok(header)
    }
}

impl Encodable for Header {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.stream_rlp(s, true);
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
