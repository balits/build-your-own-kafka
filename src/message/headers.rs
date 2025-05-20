use crate::message::api_keys::ApiKeys;
use crate::primitives::{CompactArray, Tag};

use crate::codec::WireLength;

pub struct ResponseHeaderV0 {
    pub(crate) correlation_id: i32,
}

impl ResponseHeaderV0 {
    pub fn new(correlation_id: i32) -> Self {
        Self { correlation_id }
    }

    pub fn size(&self) -> usize {
        size_of_val(self)
    }
}

#[derive(Debug)]
pub struct RequestHeaderV2 {
    pub(crate) request_api_key: ApiKeys,
    pub(crate) request_api_version: i16,
    pub(crate) correlation_id: i32,
    pub(crate) client_id: String,
    pub(crate) tag_buffer: CompactArray<Tag>,
}

impl RequestHeaderV2 {
    /// Safety
    ///
    /// this constructor does nothing to
    /// check if the api_version is anything meaningfull.
    pub fn new(
        request_api_key: i16,
        request_api_version: i16,
        correlation_id: i32,
        client_id: String,
        tag_buffer: CompactArray<Tag>,
    ) -> Self {
        Self {
            request_api_key: request_api_key.into(),
            request_api_version,
            correlation_id,
            client_id,
            tag_buffer,
        }
    }
}

/// This returns the length of the header
/// in bytes, using the strings length, rather than its
/// stack memory representation (ptr, len, cap)
impl WireLength for RequestHeaderV2 {
    fn wire_len(&self) -> usize {
        2 + 2 + 4 + 2 + self.client_id.len() + self.tag_buffer.len()
    }
}
