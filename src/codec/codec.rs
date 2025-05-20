use anyhow::{self, bail, ensure};
use bytes::{Buf, BufMut};
use tokio_util::codec;
use tracing::debug;

use super::{CustomDecoder, WireLength};

use crate::{
    message::{
        api_keys::ApiKeys,
        body::{ApiVersionRequestBody, RequestBody},
        headers::RequestHeaderV2,
        request::KafkaRequest,
        response::KafkaResponse,
    },
    primitives::{CompactArray, Tag},
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
        // not enough for message_size
        if src.len() < 4 {
            return Ok(None);
        }

        let message_size = src.get_i32();
        debug!("{}", message_size);
        if message_size < 0 {
            bail!("Message size cannot be negative ({message_size})");
        }
        let message_size = message_size as usize;
        if message_size > MAX_BODY_SIZE {
            bail!("Message size exceeds maximum size {MAX_BODY_SIZE}");
        }

        // parsing fixed sized fields of header
        if src.len() < 8 {
            // fixed sized fields of the header has not yet arrived, reserve size for them then
            // bail
            src.reserve(8);
            return Ok(None);
        }

        // after this, we can match on ApiKeys, so we know what kind of body to expect
        let request_api_key = src.get_i16();
        debug!("api key {}", request_api_key);
        let request_api_version = src.get_i16();
        debug!("api version {}", request_api_version);
        let correlation_id = src.get_i32();
        debug!("corr id {}", correlation_id);

        let client_id = super::decode_nullable_string(src)?;
        let client_id = match client_id {
            Some(str) => str,
            None => return Ok(None),
        };
        debug!("client id {}", client_id);

        // TODO: tag buffer is usually empty in codecrafters, but it should be implemnted anyways
        // but for now im just gonna leave it here
        let tag_buffer  = match CompactArray::<Tag>::decode(src, 0) {
            Ok(opt) => match opt {
                Some(val) => val,
                None => return Ok(None),
            },
            Err(e) => bail!(e),
        };
        debug!("tag buff {:?}", tag_buffer);

        let header = RequestHeaderV2::new(
            request_api_key,
            request_api_version,
            correlation_id,
            client_id,
            tag_buffer,
        );

        let body_size = message_size - header.wire_len();
        debug!("{}", body_size);
        debug!("{}", header.wire_len());

        if src.len() < body_size {
            src.reserve(body_size);
            return Ok(None);
        }

        let body = match header.request_api_key {
            ApiKeys::ApiVersions => {
                let body_inner = ApiVersionRequestBody::decode(src, body_size)?;

                if body_inner.is_none() {
                    return Ok(None);
                }

                RequestBody::ApiVersions(body_inner.unwrap())
            }
            u @ ApiKeys::UNIMPLEMENTED => bail!("Got request with unimplemented api key {u}"),
        };

        // as i32 cast is safe due to previos assertion 0 <= message_size <= MAX_BODY_SIZE
        Ok(Some(KafkaRequest::new(message_size as i32, header, body)))
    }
}
impl codec::Encoder<KafkaResponse> for KafkaCodec {
    type Error = anyhow::Error;

    fn encode(
        &mut self,
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
