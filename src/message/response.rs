use bytes::Bytes;

use crate::message::headers::ResponseHeaderV0;

pub struct KafkaResponse {
    pub(crate) message_size: i32,
    pub(crate) header: ResponseHeaderV0,
    pub(crate) body: Bytes,
}
