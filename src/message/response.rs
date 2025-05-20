use bytes::Bytes;

use crate::codec::MAX_BODY_SIZE;
use crate::message::headers::ResponseHeaderV0;

pub struct KafkaResponse {
    pub(crate) message_size: i32,
    pub(crate) header: ResponseHeaderV0,
    pub(crate) body: Bytes,
}

impl KafkaResponse {
    pub fn from_raw_parts(header: ResponseHeaderV0, body: &[u8]) -> anyhow::Result<Self> {
        let size = size_of::<ResponseHeaderV0>() + body.len();
        anyhow::ensure!(
            size < MAX_BODY_SIZE,
            "Response body size exceeded limit {MAX_BODY_SIZE}"
        );

        Ok(Self {
            message_size: size as i32,
            header,
            body: Bytes::copy_from_slice(body),
        })
    }
}
