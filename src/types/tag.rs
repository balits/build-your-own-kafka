#![allow(dead_code)]

use crate::codec::{Decoder, Encoder};
use crate::primitives::CompactArray;
use bytes::BufMut;
use kafka_macros::WireLen;

pub type TagBuf = CompactArray<Tag>;
pub const fn empty_tagbuf() -> TagBuf {
    TagBuf::new()
}

#[derive(Debug, WireLen)]
pub struct Tag {
    pub(crate) inner: u8,
}

impl Tag {
    fn new(b: u8) -> Self {
        Self { inner: b }
    }
}
impl Decoder for Tag {
    fn decode(_: &mut bytes::BytesMut, _: Option<usize>) -> anyhow::Result<Option<Self>>
    where
        Self: Sized + crate::WireLen,
    {
        unreachable!()
    }
}

impl Encoder for Tag {
    fn encode(&self, dest: &mut bytes::BytesMut) -> anyhow::Result<()> {
        dest.put_u8(self.inner);
        Ok(())
    }
}
