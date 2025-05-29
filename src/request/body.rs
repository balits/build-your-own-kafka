use anyhow::{bail, ensure};
use bytes::{self, Buf, BytesMut};
use kafka_macros::WireLen;
use tracing::trace;

use crate::primitives::{ApiKeys, CompactArray, CompactString, Tag};
use crate::{
    codec::{Decoder, WireLen},
    unwrap_decode,
};

#[derive(Debug)]
pub enum RequestBody {
    ApiVersions(ApiVersionRequestBody),
}

impl RequestBody {
    pub fn decode_by_key(key: &ApiKeys, src: &mut BytesMut, size: Option<usize>) -> anyhow::Result<Option<Self>> {
        match key {
            ApiKeys::ApiVersions => {
                let inner = unwrap_decode!(ApiVersionRequestBody::decode(src, size));

                Ok(Some(RequestBody::ApiVersions(inner)))
            }
            ApiKeys::Unimplemented => bail!("Couldnt decode body based on api key {key} as it is unimplemented!")
        }
    }
}

impl WireLen for RequestBody {
    fn wire_len(&self) -> usize {
        match self {
            RequestBody::ApiVersions(b) => b.wire_len(),
        }
    }
}

#[derive(Debug, WireLen)]
pub struct ApiVersionRequestBody {
    pub(crate) client_software_name: CompactString,
    pub(crate) client_software_version: CompactString,
    tag_buffer: CompactArray<Tag>,
}

impl Decoder for ApiVersionRequestBody {
    type Error = anyhow::Error;

    fn decode(src: &mut bytes::BytesMut, size: Option<usize>) -> Result<Option<Self>, Self::Error>
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

        let body = ApiVersionRequestBody {
            client_software_name,
            client_software_version,
            tag_buffer,
        };

        if let Some(sz) = size {
            let wl = body.wire_len();
            ensure!(
                sz == wl,
                "Size of body does not meet expectations, got: {wl}, expected: {sz}"
            );
        }

        Ok(Some(body))
    }
}
