use kafka_macros::WireLen;

use crate::{codec::Decoder, primitives::CompactString, unwrap_decode};

use super::TagBuf;

#[derive(Debug, WireLen)]
pub struct Topic {
    pub(crate) name: CompactString,
    tag_buffer: TagBuf
}

impl Topic {
    pub fn new(name: CompactString, tag_buffer: TagBuf) -> Self {
        Self {name, tag_buffer }
    }
}

impl Decoder for Topic {
    fn decode(
            src: &mut bytes::BytesMut,
            _: Option<usize>,
        ) -> anyhow::Result<Option<Self>>
        where
            Self: Sized + crate::WireLen {
        let name = unwrap_decode!(CompactString::decode(src, None));
        let tag_buffer = unwrap_decode!(TagBuf::decode(src, None));

        Ok(Some(Topic::new(name, tag_buffer)))
    }
}