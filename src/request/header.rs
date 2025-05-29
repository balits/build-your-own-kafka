use std::fmt::Debug;
use kafka_macros::WireLen;
use crate::primitives::{ApiKeys, CompactArray, Tag, NullableString};

#[derive(Debug, WireLen)]
pub struct RequestHeaderV2 {
    pub(crate) request_api_key: ApiKeys,
    pub(crate) request_api_version: i16,
    pub(crate) correlation_id: i32,
    pub(crate) client_id: NullableString,
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
        client_id: NullableString,
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
