use bytes::Bytes;
use rlp::{UntrustedRlp, RlpStream, Decodable, DecoderError};
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


impl Decodable for Block {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        if rlp.as_raw().len() != rlp.payload_info()?.total() {
            return Err(DecoderError::RlpIsTooBig);
        }
        if rlp.item_count()? != 2 {
            return Err(DecoderError::RlpIncorrectListLen);
        }
        Ok(Block {
            header: rlp.val_at(0)?,
            transactions: rlp.list_at(1)?,
        })
    }
}
