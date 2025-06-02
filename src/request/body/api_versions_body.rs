use crate::codec::{Decoder, WireLen};
use crate::primitives::*;
use crate::types::Tag;
use crate::unwrap_decode;
use anyhow;
use bytes::Buf;
use kafka_macros::WireLen;
use tracing::trace;

#[derive(Debug, WireLen)]
pub struct ApiVersionsRequestBody {
    pub(crate) client_software_name: CompactString,
    pub(crate) client_software_version: CompactString,
    tag_buffer: CompactArray<Tag>,
}

impl Decoder for ApiVersionsRequestBody {
    fn decode(src: &mut bytes::BytesMut, size: Option<usize>) -> anyhow::Result<Option<Self>>
    where
        Self: Sized + WireLen,
    {
        if let Some(sz) = size {
            if src.remaining() < sz {
                src.reserve(sz);
                return Ok(None);
            }
        }

        trace!("parsing client_software_name");
        let client_software_name = unwrap_decode!(CompactString::decode(src, None));
        trace!(client_software_name = ?client_software_name, wire_len = client_software_name.wire_len());

        trace!("parsing client_software_version");
        let client_software_version = unwrap_decode!(CompactString::decode(src, None));
        trace!(client_software_version = ?client_software_version, wire_len = client_software_version.wire_len());

        let tag_buffer = unwrap_decode!(CompactArray::decode(src, None));

        let body = ApiVersionsRequestBody {
            client_software_name,
            client_software_version,
            tag_buffer,
        };

        if let Some(sz) = size {
            let wl = body.wire_len();
            anyhow::ensure!(
                sz == wl,
                "Size of body does not meet expectations, got: {wl}, expected: {sz}"
            );
        }

        Ok(Some(body))
    }
}
