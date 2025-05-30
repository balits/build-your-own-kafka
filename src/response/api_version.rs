use bytes::{BufMut, BytesMut};
use kafka_macros::WireLen;

use crate::{
    codec::Encoder,
    primitives::{CompactArray},
    types::Tag,
};

#[derive(Debug, WireLen)]
pub struct ApiVersion {
    pub(crate) api_key: u16,
    pub(crate) min_version: u16,
    pub(crate) max_version: u16,
    pub(crate) tag_buffer: CompactArray<Tag>,
}

impl ApiVersion {
    pub fn new(api_key: u16, min_sup_version: u16, max_sup_version: u16) -> Self {
        Self {
            api_key,
            min_version: min_sup_version,
            max_version: max_sup_version,
            tag_buffer: CompactArray::new(),
        }
    }
}

impl Encoder for ApiVersion {
    fn encode(&self, dest: &mut BytesMut) -> anyhow::Result<()> {
        dest.put_u16(self.api_key);
        dest.put_u16(self.min_version);
        dest.put_u16(self.max_version);
        self.tag_buffer.encode(dest)
    }
}
