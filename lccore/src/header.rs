use ethereum_types::{Address, Bloom, H256, U256};
use bytes::Bytes;
use rlp::{Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};
use keccak_hash::{keccak, KECCAK_NULL_RLP};
use byteorder::{BigEndian, ByteOrder};

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
    pub nonce: Bytes,
}

impl PartialEq for Header {
    fn eq(&self, c: &Header) -> bool {
        self.parent_hash == c.parent_hash && self.coinbase == c.coinbase
            && self.state_root == c.state_root && self.receipts_root == c.receipts_root
            && self.transactions_root == c.transactions_root
            && self.log_bloom == c.log_bloom && self.difficulty == c.difficulty
            && self.number == c.number && self.gas_used == c.gas_used
            && self.gas_limit == c.gas_limit && self.timestamp == c.timestamp
            && self.extra_data == c.extra_data && self.mix_digest == c.mix_digest
            && self.nonce == c.nonce
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
            nonce: vec![],
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
    pub fn nonce(&self) -> u64 {
        BigEndian::read_u64(&self.nonce.as_slice()[0..8])
    }

    pub fn stream_rlp(&self, s: &mut RlpStream, with_seal: bool) {
        s.begin_list(13 + if with_seal { 2 } else { 0 });
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
        let item_count = r.item_count()?;
        if r.item_count()? != 14 {
            Err(DecoderError::RlpIncorrectListLen)
        } else {
            Ok(Header {
                parent_hash: r.val_at(0)?,
                coinbase: r.val_at(1)?,
                state_root: r.val_at(2)?,
                transactions_root: r.val_at(3)?,
                receipts_root: r.val_at(4)?,
                log_bloom: r.val_at(5)?,
                difficulty: r.val_at(6)?,
                number: r.val_at(7)?,
                gas_limit: r.val_at(8)?,
                gas_used: r.val_at(9)?,
                timestamp: r.val_at::<U256>(10)?.as_u64(),
                extra_data: r.val_at(11)?,
                mix_digest: r.val_at(12)?,
                nonce: r.val_at(13)?,
            })
        }
    }
}

impl Encodable for Header {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.stream_rlp(s, true);
    }
}

#[cfg(test)]
mod tests {
    use rustc_hex::FromHex;
    use rlp;
    use super::Header;
    use ethereum_types::{Address, Bloom, H256, U256};
    use byteorder::{BigEndian, ByteOrder};
    use bytes::Bytes;

    #[test]
    fn test_header_decode() {
        let header_rlp = "f901d8a083cafc574e1f51ba9dc0568fc617a08ea2429fb384059c972f13b19fa1c8dd55948888f1f195afa192cfee860698584c030f4c9db1a0ef1552a40b7165c3cd773806b9e0c165b75356e0314bf0706f279c729f51e017a05fe50b260da6308036625b850b5d6ced6d0a9f814c0688bc91ffb7b7a3a54b67a0bc37d79753ad738a6dac4921e57392f145d8887476de3f783dfa7edae9283e52b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008302000001832fefd8825208845506eb0780a0bd4472abb6659ebe3ee06ee4d7b72a00a9f4d001caca51342001075469aff49888a13a5a8c8f2bb1c4".from_hex().unwrap();
        let expected_header: Header = rlp::decode(&header_rlp);
        let mut nonce: Bytes = vec![0; 8];
        BigEndian::write_u64(&mut nonce, 0xa13a5a8c8f2bb1c4);
        let header = Header {
            parent_hash: H256::from(
                "83cafc574e1f51ba9dc0568fc617a08ea2429fb384059c972f13b19fa1c8dd55"
                    .from_hex()
                    .unwrap()
                    .as_slice(),
            ),
            coinbase: Address::from(
                "8888f1f195afa192cfee860698584c030f4c9db1"
                    .from_hex()
                    .unwrap()
                    .as_slice(),
            ),
            state_root: H256::from(
                "ef1552a40b7165c3cd773806b9e0c165b75356e0314bf0706f279c729f51e017"
                    .from_hex()
                    .unwrap()
                    .as_slice(),
            ),
            receipts_root: H256::from(
                "bc37d79753ad738a6dac4921e57392f145d8887476de3f783dfa7edae9283e52"
                    .from_hex()
                    .unwrap()
                    .as_slice(),
            ),
            transactions_root: H256::from(
                "5fe50b260da6308036625b850b5d6ced6d0a9f814c0688bc91ffb7b7a3a54b67"
                    .from_hex()
                    .unwrap()
                    .as_slice(),
            ),
            log_bloom: Bloom::default(),
            difficulty: U256::from(131072),
            number: 1,
            gas_used: U256::from(21000),
            gas_limit: U256::from(3141592),
            timestamp: 1426516743,
            extra_data: vec![],
            mix_digest: H256::from(
                "bd4472abb6659ebe3ee06ee4d7b72a00a9f4d001caca51342001075469aff498"
                    .from_hex()
                    .unwrap()
                    .as_slice(),
            ),
            nonce: nonce,
        };
        assert_eq!(header, expected_header);
    }
}
