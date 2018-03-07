use bytes::Bytes;
use rlp::{Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};
use header::Header;
use transaction::Transaction;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn rlp_bytes(&self, seal: bool) -> Bytes {
        let mut buf = RlpStream::new_list(3);
        self.header.stream_rlp(&mut buf, seal);
        buf.append_list(&self.transactions);
        buf.out()
    }
}

impl Encodable for Block {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.header);
        s.append_list(&self.transactions);
    }
}

impl Decodable for Block {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        if rlp.item_count()? != 2 {
            return Err(DecoderError::RlpIncorrectListLen);
        }
        Ok(Block {
            header: rlp.val_at(0)?,
            transactions: rlp.list_at(1)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use rustc_hex::FromHex;
    use rlp;
    use super::Block;

    #[test]
    fn test_block_decode() {
        let block_rlp = "f9023ef901d8a083cafc574e1f51ba9dc0568fc617a08ea2429fb384059c972f13b19fa1c8dd55948888f1f195afa192cfee860698584c030f4c9db1a0ef1552a40b7165c3cd773806b9e0c165b75356e0314bf0706f279c729f51e017a05fe50b260da6308036625b850b5d6ced6d0a9f814c0688bc91ffb7b7a3a54b67a0bc37d79753ad738a6dac4921e57392f145d8887476de3f783dfa7edae9283e52b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008302000001832fefd8825208845506eb0780a0bd4472abb6659ebe3ee06ee4d7b72a00a9f4d001caca51342001075469aff49888a13a5a8c8f2bb1c4f861f85f800a82c35094095e7baea6a6c7c4c2dfeb977efac326af552d870a801ba09bea4c4daac7c7c52e093e6a4c35dbbcf8856f1af7b059ba20253e70848d094fa08a8fae537ce25ed8cb5af9adac3f141af69bd515bd2ba031522df09b97dd72b1".from_hex().unwrap();
        let block: Block = rlp::decode(&block_rlp);
        assert_eq!(block.transactions.len(), 1);
    }
}
