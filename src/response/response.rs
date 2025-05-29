use bytes::{BufMut, BytesMut};
use kafka_macros::WireLen;

use crate::codec::Encoder;
use super::headers::ResponseHeaderV0;
use super::body::ResponseBody;

#[derive(Debug, WireLen)]
pub struct KafkaResponse {
    pub(crate) message_size: i32,
    pub(crate) header: ResponseHeaderV0,
    pub(crate) body: ResponseBody,
}

impl KafkaResponse {
    pub fn new(message_size: i32, header: ResponseHeaderV0, body: ResponseBody) -> Self {
        Self {message_size, header, body}
    }
}



impl Encoder for KafkaResponse {
    /// This is the top level call to decode
    fn encode(&self, dest: &mut BytesMut) -> anyhow::Result<()> {
        dest.put_i32(self.message_size);
        dest.put_i32(self.header.correlation_id);

        let ResponseBody::ApiVersion(ref body) = self.body;
        dest.put_u16(body.error_code);
        dest.put_u8(body.api_versions.len() as u8 + 1); // since its N + 1 for N elements
        
        for ver in body.api_versions.iter() {
            dest.put_u16(ver.api_key);
            dest.put_u16(ver.min_version);
            dest.put_u16(ver.max_version);
            dest.put_u8(0); // tag buff
        }
        dest.put_u32(body.throttle_time);
        dest.put_u8(0); // tag buff

        Ok(())
    }
}