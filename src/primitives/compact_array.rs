use bytes::Buf;

use crate::codec::{lib::Encoder, Decoder, WireLen};
use std::{fmt::Debug, u32, usize};

use super::UVarint;

/// # Kafka protocol
///
/// Represents a sequence of objects of a given type `T`.
/// Type `T` can be either a primitive type (e.g. STRING) or a structure.
/// First, the length N + 1 is given as an UNSIGNED_VARINT. Then N instances of type `T` follow.
/// A null array is represented with a length of 0. In protocol documentation an array of `T` instances is referred to as `[T]`.
///
/// # Safety
///
/// Only null arrays are supported as of yet, decoding into this type
/// with a length anything other than zero panics
#[derive(Debug)]
pub struct CompactArray<T: WireLen> {
    inner: Vec<T>, // Vec for prototyping
}

impl<T: WireLen> Default for CompactArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: WireLen> CompactArray<T> {
    pub fn new() -> Self {
        Self {
            // No allloc
            inner: Vec::new(),
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            // No allloc
            inner: Vec::with_capacity(cap),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn push(&mut self, val: T) {
        self.inner.push(val);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        return self.inner.iter()
    }
}

impl<T: WireLen> WireLen for CompactArray<T> {
    fn wire_len(&self) -> usize {
        // This down here is all incorrect, I was blind and they do infact
        // send 0x00 indicating an empty compact array as the tag buffers
        // this did infact take 3 days to figure out.
        //
        // current implementation assumes codecrafters does not even
        // send the length prefix for an empty tag buffer (whre compact array is used)
        if self.len() == 0 {
            UVarint::wire_len_of(0)
        } else {
            assert!(self.len() <= u32::MAX as usize, "Compact array holds more than u32::MAX, the max size of uvarint");
            UVarint::wire_len_of(self.len() as u32) + self.len() * self.inner[0].wire_len()
        }
    }
}

impl<T: WireLen> Decoder for CompactArray<T> {
    type Error = anyhow::Error;

    fn decode(src: &mut bytes::BytesMut, _: Option<usize>) -> Result<Option<Self>, Self::Error> {
        // CompactArray is used for storing tags, we dont care about it
        // since its usually empty anyways
        //
        // let _let_plus_one = match UVarint::decode(src, None)? {
        //     Some(v) => v.0,
        //     None => return Ok(None),
        // };
        if src.remaining() < 1 {
            src.reserve(1);
            return Ok(None);
        }

        let zero = src.get_u8();
        anyhow::ensure!(zero == 0, "Tag buffers (and therefore CompactArrays) are expected to be zero length");

        Ok(Some(CompactArray::new()))
    }
}

impl<T: WireLen + Encoder> Encoder for CompactArray<T> {
    fn encode(&self, dest: &mut bytes::BytesMut) ->  anyhow::Result<()> {
        let u = UVarint(self.len() as u32);
        UVarint::encode(&u, dest)?;
        for e in &self.inner {
            Encoder::encode(e, dest)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::*;
    use super::super::super::message::response::ApiVersion;
    
    #[test]
    fn test_encode_simple() {
        let mut api_versions = CompactArray::new();
        api_versions.push(ApiVersion::new(1,0,17));

        let mut buf = BytesMut::with_capacity(api_versions.wire_len());
        api_versions.encode(&mut buf).unwrap();

        assert_eq!(api_versions.wire_len(), buf.len());

        let c = buf.freeze();
        let mut buf = c.clone();
        
        assert_eq!(api_versions.len() as u8, buf.get_u8());
        assert_eq!(1, buf.get_u16());
        assert_eq!(0, buf.get_u16());
        assert_eq!(17,buf.get_u16());
        assert_eq!(0, buf.get_u8());
        assert!(!buf.has_remaining());

        println!("{:?}", buf);
    }
}
