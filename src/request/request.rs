use anyhow::bail;
use bytes::Buf;
use kafka_macros::WireLen;
use tracing::{info, trace};

use super::body::RequestBody;
use super::header::RequestHeaderV2;
use crate::codec::{Decoder, MAX_MESSAGE_SIZE, WireLen};
use crate::unwrap_decode;

#[derive(WireLen, Debug)]
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

impl Decoder for KafkaRequest {
    fn decode(src: &mut bytes::BytesMut, _: Option<usize>) -> anyhow::Result<Option<Self>>
    where
        Self: Sized + crate::WireLen,
    {
        if src.is_empty() {
            trace!("Decode called on empty buffer");
        }

        if src.len() < 4 {
            return Ok(None);
        }

        let message_size = src.get_i32();
        trace!("message_size");

        if message_size < 0 {
            bail!("Message size cannot be negative ({message_size})");
        }

        let message_size = message_size as usize;

        if message_size > MAX_MESSAGE_SIZE {
            bail!("Message size exceeds maximum size {MAX_MESSAGE_SIZE}");
        }

        if src.len() < 8 {
            // fixed sized fields of the header has not yet arrived, reserve size for them then
            src.reserve(8);
            return Ok(None);
        }

        let header = unwrap_decode!(RequestHeaderV2::decode(src, None));
        let body_size = message_size - header.wire_len();

        if src.remaining() < body_size {
            trace!(
                "not enough space for body. body.len(): {}, src.len() = {}",
                body_size,
                src.remaining()
            );
            src.reserve(body_size);
            return Ok(None);
        }

        let body = unwrap_decode!(RequestBody::decode_by_key(
            &header.request_api_key,
            src,
            Some(body_size)
        ));

        // an i32 cast is safe due to previos assertion 0 <= message_size <= MAX_BODY_SIZE
        let req = KafkaRequest::new(message_size as i32, header, body);
        info!(
            "Parsed Request! Bytes remainging in buffer: {}",
            src.remaining()
        );
        Ok(Some(req))
    }
}
