use bytes;

use crate::codec::{CustomDecoder,WireLength};
use crate::primitives::{CompactArray, CompactString, Tag};

/// A common trait for request bodies which allows
/// for debugging, getting the size of bytes it takes
/// for encoding-decoding, and decoding
pub trait Body: std::fmt::Debug + WireLength + CustomDecoder {}

#[derive(Debug)]
pub enum RequestBody {
    ApiVersions(ApiVersionRequestBody),
}

#[derive(Debug)]
pub struct ApiVersionRequestBody {
    pub(crate) client_id: CompactString,
    pub(crate) client_software_version: CompactString,
    pub(crate) tag_buffer: CompactArray<Tag>,
}

impl WireLength for ApiVersionRequestBody {
    fn wire_len(&self) -> usize {
        self.client_id.wire_len()
            + self.client_software_version.wire_len()
            + self.tag_buffer.wire_len()
    }
}

impl CustomDecoder for ApiVersionRequestBody {
    type Error = anyhow::Error;

    fn decode(src: &mut bytes::BytesMut, size: usize) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized + WireLength,
    {
        if src.len() < size {
            src.reserve(size);
            return Ok(None);
        }

        // TODO: macro to hanlde Result<Option<Item>, Error>>
        let client_id = CompactString::decode(src, 0);
        let client_software_version = CompactString::decode(src, 0);
        let tag_buffer = CompactArray::<Tag>::decode(src, 0);

        todo!()
    }
}
