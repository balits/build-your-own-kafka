use anyhow::{self, bail, ensure};
use bytes::{Buf, BufMut};
use tokio_util::codec;
use tracing::{info, span, trace, Level};

use super::{CustomDecoder, WireLen};

use crate::{
    message::{
        api_keys::ApiKeys,
        body::{ApiVersionRequestBody, RequestBody},
        headers::RequestHeaderV2,
        request::KafkaRequest,
        response::KafkaResponse,
    },
    primitives::{CompactArray, NullableString},
    unwrap_decode,
};

pub const MAX_BODY_SIZE: usize = 1024;

pub struct KafkaCodec {}
impl Default for KafkaCodec {
    fn default() -> Self {
        Self {}
    }
}

impl codec::Decoder for KafkaCodec {
    type Item = KafkaRequest;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            return Ok(None);
        }

        let message_size = src.get_i32();
        trace!(message_size = message_size);

        if message_size < 0 {
            bail!("Message size cannot be negative ({message_size})");
        }

        let message_size = message_size as usize;
        if message_size > MAX_BODY_SIZE {
            bail!("Message size exceeds maximum size {MAX_BODY_SIZE}");
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

        // As far as I know, they do not even send 0x00, 0x00 as zero length for
        // empty tag buffer
        // let tag_buffer = unwrap_decode!(CompactArray::<Tag>::decode(src, None));
        let tag_buffer = CompactArray::new();

        {
            trace!(wire_len_api_key = request_api_key.wire_len());
            trace!(wire_len_api_key_version = request_api_version.wire_len());
            trace!(wire_len_correlation_id = correlation_id.wire_len());
            trace!(wire_len_client_id = client_id.wire_len());
            trace!(wire_len_tag_buffer = tag_buffer.wire_len());
            trace!(
                sum = request_api_key.wire_len()
                    + request_api_version.wire_len()
                    + correlation_id.wire_len()
                    + client_id.wire_len()
                    + tag_buffer.wire_len()
            );
        }

        let header = RequestHeaderV2::new(
            request_api_key,
            request_api_version,
            correlation_id,
            client_id,
            tag_buffer,
        );

        let body_size = message_size - header.wire_len();

        info!(
            "body_size {} = message_size {} - header.wire_len {}",
            body_size,
            message_size,
            header.wire_len()
        );

        if src.len() < body_size {
            src.reserve(body_size);
            return Ok(None);
        }

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
        Ok(Some(req))
    }
}

impl codec::Encoder<KafkaResponse> for KafkaCodec {
    type Error = anyhow::Error;

    fn encode(&mut self,
        item: KafkaResponse,
        dst: &mut bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        ensure!(
            item.message_size > 0,
            "KafkaRequest had negative message size"
        );
        ensure!(
            (item.message_size as usize) < MAX_BODY_SIZE,
            "KafkaRequest's message size is bigger than allowed {MAX_BODY_SIZE}"
        );
        ensure!(item.message_size as usize == item.body.len() + item.header.size(),
            "KafkRequest's message_size and actual length of body is not equal (expected {}, got {})", item.message_size, item.body.len());

        dst.reserve(4);
        dst.put_i32(item.message_size);

        dst.put_i32(item.header.correlation_id);

        dst.reserve(item.message_size as usize);
        dst.extend(item.body);

        Ok(())
    }
}
