use bytes::{BufMut, BytesMut};
use kafka_macros::WireLen;

use super::body::ResponseBody;
use super::headers::ResponseHeaderV0;
use crate::codec::Encoder;

#[derive(Debug, WireLen)]
pub struct KafkaResponse {
    pub(crate) message_size: i32,
    pub(crate) header: ResponseHeaderV0,
    pub(crate) body: ResponseBody,
}

impl KafkaResponse {
    pub fn new(message_size: i32, header: ResponseHeaderV0, body: ResponseBody) -> Self {
        Self {
            message_size,
            header,
            body,
        }
    }
}

impl Encoder for KafkaResponse {
    /// This is the top level call to decode
    fn encode(&self, dest: &mut BytesMut) -> anyhow::Result<()> {
        dest.put_i32(self.message_size);
        dest.put_i32(self.header.correlation_id);
        self.body.encode(dest)?;
        Ok(())
    }
}
