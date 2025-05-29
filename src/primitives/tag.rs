#![allow(dead_code)]

use bytes::BufMut;
use kafka_macros::WireLen;

use crate::codec::Encoder;

#[derive(Debug, WireLen)]
pub struct Tag {
    pub(crate) inner: u8,
}

impl Tag {
    fn new(b: u8) -> Self {
        Self { inner: b }
    }
}

impl Encoder for Tag {
    fn encode(&self, dest: &mut bytes::BytesMut) ->  anyhow::Result<()> {
        dest.put_u8(self.inner);        
        Ok(())
    }
}
