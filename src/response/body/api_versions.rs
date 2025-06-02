use crate::{
    codec::Encoder,
    primitives::CompactArray,
    types::{ApiVersion, Tag},
};
use bytes::{BufMut, BytesMut};
use kafka_macros::WireLen;
use std::fmt::Debug;

#[derive(Debug, WireLen)]
pub struct ApiVersionsResponseBody {
    pub(crate) error_code: u16,
    pub(crate) api_versions: CompactArray<ApiVersion>,
    pub(crate) throttle_time: u32,
    pub(crate) tag_buffer: CompactArray<Tag>,
}

impl ApiVersionsResponseBody {
    pub fn new(
        error_code: u16,
        api_versions: CompactArray<ApiVersion>,
        throttle_time: u32,
    ) -> Self {
        Self {
            error_code,
            api_versions,
            throttle_time,
            tag_buffer: CompactArray::new(),
        }
    }
}

impl Encoder for ApiVersionsResponseBody {
    fn encode(&self, dest: &mut BytesMut) -> anyhow::Result<()> {
        dest.put_u16(self.error_code);
        self.api_versions.encode(dest)?;
        dest.put_u32(self.throttle_time);
        self.tag_buffer.encode(dest)?;
        Ok(())
    }
}
