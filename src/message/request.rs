use bytes::Bytes;

use super::headers::RequestHeaderV2;

#[derive(Debug)]
pub struct KafkaRequest {
    pub message_size: i32,
    pub header: RequestHeaderV2,
    pub body: Bytes,
}
