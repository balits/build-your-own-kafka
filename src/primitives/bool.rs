use bytes::BufMut;

use crate::{WireLen, codec::Encoder};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Bool {
    False = 0,
    True = 1,
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        if value { Bool::True } else { Bool::False }
    }
}

impl Bool {
    pub fn is_true(&self) -> bool {
        *self == Bool::True
    }
    pub fn is_false(&self) -> bool {
        *self == Bool::False
    }
}

impl WireLen for Bool {
    fn wire_len(&self) -> usize {
        (*self as u8).wire_len()
    }
}

impl Encoder for Bool {
    fn encode(&self, dest: &mut bytes::BytesMut) -> anyhow::Result<()> {
        dest.put_u8(*self as u8);
        Ok(())
    }
}
