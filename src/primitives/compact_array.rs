use std::fmt::Debug;
use crate::codec::{WireLength, CustomDecoder};

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
pub struct CompactArray<T> {
    inner: Vec<T>, // Vec for prototyping
}

impl<T> CompactArray<T> {
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
}

impl<T> WireLength for CompactArray<T> {
    fn wire_len(&self) -> usize {
        return size_of::<i16>() + self.len();
    }
}

impl<T> CustomDecoder for CompactArray<T> {
    type Error = anyhow::Error;

    fn decode(src: &mut bytes::BytesMut, _size: usize) -> Result<Option<Self>, Self::Error> {
        // CompactArray is used for storing tags, we dont care about it
        // since its usually empty anyways

        let let_plus_one = match UVarint::decode(src, 0)? {
            Some(v) => v.0,
            None => return Ok(None),
        };
        let len = (let_plus_one - 1) as usize;

        anyhow::ensure!(
            len == 0,
            "COMPACT ARRAY WITH NON ZERO LENGTH IN SOURCE BUFFER"
        );
        Ok(Some(CompactArray::new()))
    }
}
