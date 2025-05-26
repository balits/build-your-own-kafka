use std::fmt::Debug;

use anyhow::{bail, ensure, Context};
use bytes::Buf;

use crate::{
    codec::{CustomDecoder, WireLen},
    primitives::MAX_STRING_SIZE,
};

#[derive(Debug)]
pub struct NullableString {
    inner: Option<String>,
}

impl NullableString {
    pub const MAX: usize = i16::MAX as usize;

    /// Creates a new NullableString from a given str slice.
    /// Returns a null variant if the argument is either empty, or is larger
    /// than the max size of the length prefix, i.e. a 2 byte
    /// signed integer, as specified by the `[kafka docs](https://kafka.apache.org/protocol.html)`
    pub fn from_non_empty_str(value: &str) -> Self {
        let inner = if value.len() != 0 && value.len() < Self::MAX {
            Some(value.to_string())
        } else {
            None
        };

        Self { inner }
    }

    pub fn null() -> Self {
        Self { inner: None }
    }
}

impl WireLen for NullableString {
    fn wire_len(&self) -> usize {
        // NullableStrings always need 2 bytes for the
        // length prefix, which is -1 if the string is null
        let length = size_of::<i16>();

        if let Some(ref s) = self.inner {
            assert!(
                s.len() != 0 && s.len() < Self::MAX,
                "invalid size of string"
            );
            length + s.len()
        } else {
            length
        }
    }
}

impl CustomDecoder for NullableString {
    type Error = anyhow::Error;
    fn decode(src: &mut bytes::BytesMut, _: Option<usize>) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized + WireLen,
    {
        if src.len() < 2 {
            // not enough space for length,
            // reserve and signal to the callers that the operation failed
            // but no error happend, so we can wait on the next frame
            src.reserve(2);
            return Ok(None);
        }

        let len = src.get_i16();

        if len == -1 {
            return Ok(Some(NullableString::null()));
        }

        if len == 0 {
            bail!("NullableString with length 0 is not allowed")
        }

        ensure!(len > 0, "Negative ammount of INT16 (other than -1)");
        ensure!(
            (len as usize) < MAX_STRING_SIZE,
            "client id length is bigger than allowed {MAX_STRING_SIZE}"
        );

        let len = len as usize;
        if src.remaining() < len {
            src.reserve(len);
            return Ok(None);
        }

        let data = src.split_to(len);

        let ns = NullableString::from_non_empty_str(
            std::str::from_utf8(&data).context("Invalid utf8 bytes")?,
        );

        Ok(Some(ns))
    }
}
