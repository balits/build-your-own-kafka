use crate::codec::{Encoder, WireLen};
use bytes::{BufMut, BytesMut};
use kafka_macros::WireLen;
use crate::primitives::{CompactArray, Tag};
use super::api_version::ApiVersion;

#[derive(Debug)]
pub enum ResponseBody {
    ApiVersion(ApiVersionResponseBody)
}

impl WireLen for ResponseBody {
    fn wire_len(&self) -> usize {
        match self {
            ResponseBody::ApiVersion(body) => body.wire_len()
        }
    }
}

impl Encoder for ResponseBody {
    fn encode(&self, dest: &mut BytesMut) ->  anyhow::Result<()> {
        match self {
            ResponseBody::ApiVersion(body) => body.encode(dest)
        }
    }
}

#[derive(Debug, WireLen)]
pub struct ApiVersionResponseBody {
    pub(crate) error_code: u16,
    pub(crate) api_versions: CompactArray<ApiVersion>,
    pub(crate) throttle_time: u32,
    pub(crate) tag_buffer: CompactArray<Tag>
}

impl ApiVersionResponseBody {
    pub fn new(error_code: u16, api_versions: CompactArray<ApiVersion>, throttle_time: u32) -> Self {
        Self { error_code, api_versions, throttle_time, tag_buffer: CompactArray::new() }
    }
}

impl Encoder for ApiVersionResponseBody {
    fn encode(&self, dest: &mut BytesMut) ->  anyhow::Result<()> {
        dest.put_u16(self.error_code);
        self.api_versions.encode(dest)?;
        dest.put_u32(self.throttle_time);
        self.tag_buffer.encode(dest)?;
        Ok(())
    }
}