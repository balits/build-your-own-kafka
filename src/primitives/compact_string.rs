use anyhow::Context;
use tracing::debug;

use crate::{
    codec::{CustomDecoder, WireLength},
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

impl WireLength for CompactString {
    fn wire_len(&self) -> usize {
        let uv = UVarint(self.0.len() as u32 + 1);
        uv.wire_len() + self.0.len()
    }
}

impl CustomDecoder for CompactString {
    type Error = anyhow::Error;

    fn decode(src: &mut bytes::BytesMut, _size: usize) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized + WireLength,
    {
        let len_plus_one = match UVarint::decode(src, 0)? {
            Some(v) => v.0,
            None => return Ok(None),
        };
        if len_plus_one == 0 {
            return Ok(Some(CompactString("".into())));
        }

        let len = (len_plus_one - 1) as usize;

        anyhow::ensure!(
            (len_plus_one as usize) < MAX_STRING_SIZE,
            "client id length is bigger than allowed {MAX_STRING_SIZE}"
        );

        debug!("CompactString: len {}", len);

        if src.len() < len {
            src.reserve(len);
            return Ok(None);
            // bail!(
            //     "Buffer doesnt have enough bytes to read bytes to string (expected {}, got {})",
            //     len,
            //     buf.len()
            // )
        }

        let data = src.split_to(len);
        debug!("CompactString: raw data: {:?}", data);

        Ok(Some(
            String::from_utf8(data.to_vec())
                .context("Parsing compact string with len {len}")?
                .into(),
        ))
    }
}
