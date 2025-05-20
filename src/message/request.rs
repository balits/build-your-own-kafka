use super::{body::RequestBody, headers::RequestHeaderV2};

#[derive(Debug)]
pub struct KafkaRequest {
    pub message_size: i32,
    pub header: RequestHeaderV2,
    pub body: RequestBody,
}

impl KafkaRequest {
    pub fn new(message_size: i32, header: RequestHeaderV2, body: RequestBody) -> Self {
        Self {
            message_size,
            header,
            body,
        }
    }
}
