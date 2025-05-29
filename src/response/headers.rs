use bytes::BufMut;
use kafka_macros::WireLen;
use crate::codec::Encoder;

#[derive(Debug, WireLen)]
pub struct ResponseHeaderV0 {
    pub(crate) correlation_id: i32,
}

impl ResponseHeaderV0 {
    pub fn new(correlation_id: i32) -> Self {
        Self { correlation_id }
    }
}

impl Encoder for ResponseHeaderV0 {
    fn encode(&self, dest: &mut bytes::BytesMut) ->  anyhow::Result<()> {
        dest.put_i32(self.correlation_id);
        Ok(())
    }
}