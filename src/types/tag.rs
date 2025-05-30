#![allow(dead_code)]

use bytes::BufMut;
use kafka_macros::WireLen;
use crate::codec::{Decoder, Encoder};
use crate::primitives::CompactArray; 

pub type TagBuf = CompactArray<Tag>;

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
    fn decode(
            _: &mut bytes::BytesMut,
            _: Option<usize>,
        ) -> anyhow::Result<Option<Self>>
        where
            Self: Sized + crate::WireLen {
        unreachable!()
    }
}

impl Encoder for Tag {
    fn encode(&self, dest: &mut bytes::BytesMut) -> anyhow::Result<()> {
        dest.put_u8(self.inner);
        Ok(())
    }
}

