// use std::fmt::Debug;

use anyhow::Context;
use tracing::trace;

use crate::{
    codec::{Decoder, WireLen},
    primitives::MAX_STRING_SIZE,
};

use super::UVarint;

#[derive(Debug)]
pub struct CompactString(pub String);

impl From<String> for CompactString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl WireLen for CompactString {
    fn wire_len(&self) -> usize {
        let uv = UVarint(self.0.len() as u32 + 1);
        uv.wire_len() + self.0.len()
    }
}

impl Decoder for CompactString {
    type Error = anyhow::Error;

    fn decode(src: &mut bytes::BytesMut, _: Option<usize>) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized + WireLen,
    {
        let len_plus_one = match UVarint::decode(src, None)? {
            Some(v) => v.0,
            None => return Ok(None),
        };
        trace!(len_plus_one = len_plus_one);

        if len_plus_one == 0 {
            return Ok(Some(CompactString("".into())));
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

        let data = src.split_to(len);
        trace!("Got data with len {}: data: {:?}", len, data);
        trace!("Remaining data: {:?}", src);

        Ok(Some(
            String::from_utf8(data.to_vec())
                .context("Parsing compact string with len {len}")?
                .into(),
        ))
    }
}
