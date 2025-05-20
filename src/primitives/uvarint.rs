use anyhow;
use bytes::Buf;

use crate::codec::{CustomDecoder, WireLength};

pub struct UVarint(pub u32);

impl CustomDecoder for UVarint {
    type Error = anyhow::Error;

    fn decode(src: &mut bytes::BytesMut, _size: usize) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized + WireLength,
    {
        let mut result = 0u32;

        for i in 0..5 {
            if !src.has_remaining() {
                // might be bad practive, instead of returning Ok(None)
                anyhow::bail!("Not enough bytes to decode uvarint");
            }

            let byte = src.get_u8();
            result |= ((byte & 0x7F) as u32) << (7 * i);

            if (byte & 0x80) == 0 {
                return Ok(Some(UVarint(result)));
            }
        }

        // might be bad practive, instead of returning Ok(None)
        anyhow::bail!("Uvarint too long (more than 5 bytes for u32)")
    }
}

impl WireLength for UVarint {
    fn wire_len(&self) -> usize {
        match self.0 {
            0..=0x7F => 1,                // 7 bits
            0x80..=0x3FFF => 2,           // 14 bits
            0x4000..=0x1F_FFFF => 3,      // 21 bits
            0x20_0000..=0x0FFF_FFFF => 4, // 28 bits
            _ => 5,                       // 32 bits;
        }
    }
}
