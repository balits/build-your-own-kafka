use crate::{
    codec::Decoder,
    primitives::{CompactArray, NullableString},
    types::{ApiKeys, Tag},
    unwrap_decode,
};
use bytes::Buf;
use kafka_macros::WireLen;
use std::fmt::Debug;

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

impl Decoder for RequestHeaderV2 {
    fn decode(src: &mut bytes::BytesMut, _: Option<usize>) -> anyhow::Result<Option<Self>>
    where
        Self: Sized + crate::WireLen,
    {
        if src.remaining() < 8 {
            src.reserve(8);
            return Ok(None);
        }
        let request_api_key = src.get_i16();
        let request_api_version = src.get_i16();
        let correlation_id = src.get_i32();

        let client_id = unwrap_decode!(NullableString::decode(src, None));
        let tag_buffer = unwrap_decode!(CompactArray::<Tag>::decode(src, None));

        let h = RequestHeaderV2::new(
            request_api_key,
            request_api_version,
            correlation_id,
            client_id,
            tag_buffer,
        );

        Ok(Some(h))
    }
}
