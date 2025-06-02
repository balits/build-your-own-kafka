use anyhow::Context;
use bytes::BufMut;

use crate::{
    codec::{Decoder, Encoder, WireLen},
    primitives::MAX_STRING_SIZE,
    unwrap_decode,
};

use super::UVarint;

#[derive(Debug, Clone)]
pub struct CompactString(pub String);

impl Default for CompactString {
    fn default() -> Self {
        Self::new()
    }
}

impl CompactString {
    pub const fn new() -> Self {
        Self(String::new())
    }
}

impl<S> From<S> for CompactString
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

impl WireLen for CompactString {
    fn wire_len(&self) -> usize {
        let uv = UVarint(self.0.len() as u32 + 1);
        uv.wire_len() + self.0.len()
    }
}

impl Encoder for CompactString {
    fn encode(&self, dest: &mut bytes::BytesMut) -> anyhow::Result<()> {
        let u = UVarint((self.0.len() + 1) as u32);
        UVarint::encode(&u, dest)?;
        for b in self.0.as_bytes() {
            dest.put_u8(*b);
        }

        Ok(())
    }
}

impl Decoder for CompactString {
    fn decode(src: &mut bytes::BytesMut, _: Option<usize>) -> anyhow::Result<Option<Self>>
    where
        Self: Sized + WireLen,
    {
        let uv = unwrap_decode!(UVarint::decode(src, None));
        let len_plus_one = uv.0;

        if len_plus_one == 0 {
            return Ok(Some(CompactString::new()));
        }

        let len = (len_plus_one - 1) as usize;

        anyhow::ensure!(
            (len_plus_one as usize) < MAX_STRING_SIZE,
            "client id length is bigger than allowed {MAX_STRING_SIZE}"
        );

        if src.len() < len {
            src.reserve(len);
            return Ok(None);
        }

        let data = src.split_to(len).to_vec();

        let raw = String::from_utf8(data).context("Parsing compact string")?;

        Ok(Some(raw.into()))
    }
}
