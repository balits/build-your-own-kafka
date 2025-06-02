use crate::{request::KafkaRequest, types::TagBuf};
use kafka_macros::{Encoder, WireLen};

#[derive(Debug, WireLen, Encoder)]
pub struct ResponseHeaderV0 {
    pub(crate) correlation_id: i32,
}

impl ResponseHeaderV0 {
    pub fn new(correlation_id: i32) -> Self {
        Self { correlation_id }
    }

    /// Creates a new `ResponseHeaderV0`, with the same correlation id as in the request's header
    /// This is a shorthand for
    /// ```
    /// let request = {...};
    /// ResponseHeaderV0::new(request.header.correlation_id)
    /// ```
    pub fn respond(request: &KafkaRequest) -> Self {
        Self {
            correlation_id: request.header.correlation_id,
        }
    }
}

#[derive(Debug, WireLen, Encoder)]
pub struct ResponseHeaderV1 {
    pub(crate) correlation_id: i32,
    tag_buffer: TagBuf
}

impl ResponseHeaderV1 {
    pub fn new(correlation_id: i32) -> Self {
        Self { correlation_id, tag_buffer: TagBuf::new() }
    }

    /// Creates a new `ResponseHeaderV0`, with the same correlation id as in the request's header
    /// This is a shorthand for
    /// ```
    /// let request = {...};
    /// ResponseHeaderV0::new(request.header.correlation_id)
    /// ```
    pub fn respond(request: &KafkaRequest) -> Self {
        Self::new(request.header.correlation_id)
    }
}
