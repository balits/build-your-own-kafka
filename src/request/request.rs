use anyhow::bail;
use bytes::Buf;
use kafka_macros::WireLen;
use tracing::{info, trace};

use crate::request::ApiVersionRequestBody;
use crate::response::ApiVersion;
use crate::unwrap_decode;
use crate::codec::{WireLen, Decoder, MAX_MESSAGE_SIZE};
use crate::primitives::{ApiKeys, CompactArray, NullableString, Tag};
use super::body::RequestBody;
use super::header::RequestHeaderV2;

#[derive(WireLen)]
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
    type Error = anyhow::Error;
    
    fn decode(
            src: &mut bytes::BytesMut,
            _: Option<usize>,
        ) -> Result<Option<Self>, Self::Error>
        where
            Self: Sized + crate::WireLen {

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
        let request_api_key = src.get_i16();
        let request_api_version = src.get_i16();
        let correlation_id = src.get_i32();

        let client_id = unwrap_decode!(NullableString::decode(src, None));

        let tag_buffer = unwrap_decode!(CompactArray::<Tag>::decode(src, None));

        let header = RequestHeaderV2::new(
            request_api_key,
            request_api_version,
            correlation_id,
            client_id,
            tag_buffer,
        );

        let body_size = message_size - header.wire_len();

        if src.remaining() < body_size {
            trace!("not enough space for body, src.len() = {}", src.remaining());
            src.reserve(body_size);
            return Ok(None);
        }

        trace!("len: {}, data: {:X}", src.len(), &src);

        let body = match header.request_api_key {
            ApiKeys::ApiVersions => {
                let body_inner =
                    unwrap_decode!(ApiVersionRequestBody::decode(src, Some(body_size)));
                RequestBody::ApiVersions(body_inner)
            }
            u @ ApiKeys::UNIMPLEMENTED => bail!("Got request with unimplemented api key {u}"),
        };

        // an i32 cast is safe due to previos assertion 0 <= message_size <= MAX_BODY_SIZE
        let req = KafkaRequest::new(message_size as i32, header, body);
        info!("Parsed Request! Bytes remainging in buffer: {}", src.remaining());
        Ok(Some(req))
    }
}
