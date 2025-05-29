use crate::codec::{lib::Encoder, Decoder, WireLen};
use bytes::{Buf, BufMut};
use thiserror::Error;
use tracing::trace;



pub struct UVarint(pub(crate) u32);

impl WireLen for UVarint {
    fn wire_len(&self) -> usize {
       UVarint::wire_len_of(self.0)
    }
}

impl UVarint {
    pub const fn wire_len_of(num: u32) -> usize {
        match num {
            0..=0x7F => 1,                // 7 bits
            0x80..=0x3FFF => 2,           // 14 bits
            0x4000..=0x1F_FFFF => 3,      // 21 bits
            0x20_0000..=0x0FFF_FFFF => 4, // 28 bits
            _ => 5,                       // 32 bits;
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum UVarintDecodeError {
    #[error("overflow for u32")]
    Overflow,
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
}

impl Decoder for UVarint {
    type Error = UVarintDecodeError;

    /// see (wiki of uvarint)[https://en.wikipedia.org/wiki/LEB128]
    fn decode(src: &mut bytes::BytesMut, _: Option<usize>) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized + WireLen,
    {
        let mut result = 0u32;
        let mut shift = 0;

        let mut count = 0;
        while src.has_remaining() {
            count += 1;
            let byte = src.get_u8();

            trace!(" {}. iter:  byte = {:x}", count, byte);
            let value = (byte & 0x7F) as u32;
            trace!(" {}. iter: value = {:x}", count, value);
            if shift >= 32 && value != 0 {
                return Err(UVarintDecodeError::Overflow);
            }
            result |= value << shift;
            trace!(" {}. iter: result = {}", count, result);
            if (byte & 0x80) == 0 {
                return Ok(Some(UVarint(result)));
            }
            shift += 7;
        }

        Err(UVarintDecodeError::UnexpectedEndOfInput)
    }
}

impl Encoder for UVarint {
    /// see wiki https://en.wikipedia.org/wiki/LEB128#Encode_signed_32-bit_integer
    fn encode(&self, dest: &mut bytes::BytesMut) ->  anyhow::Result<()> {
        let mut value = self.0;
        while value >= 0x80 {
            dest.put_u8((value as u8 & 0x7F) | 0x80);
            value >>= 7;
        }
        dest.put_u8(value as u8);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_decode_uvarint_simple() {
        let mut buf = BytesMut::from(&[0x00][..]);
        let result = UVarint::decode(&mut buf, None);
        assert_eq!(0, result.unwrap().unwrap().0);

        let mut buf = BytesMut::from(&[0x01][..]);
        let result = UVarint::decode(&mut buf, None);
        assert_eq!(1, result.unwrap().unwrap().0);

        let mut buf = BytesMut::from(&[0x7F][..]);
        let result = UVarint::decode(&mut buf, None);
        assert_eq!(127, result.unwrap().unwrap().0);

        let mut buf = BytesMut::from(&[0x80, 0x01][..]);
        let result = UVarint::decode(&mut buf, None);
        assert_eq!(128, result.unwrap().unwrap().0);

        let mut buf = BytesMut::from(&[0xFF, 0x01][..]);
        let result = UVarint::decode(&mut buf, None);
        assert_eq!(255, result.unwrap().unwrap().0);
    }

    #[test]
    fn test_decode_uvarint_multibyte() {
        let mut buf = BytesMut::from(&[0xAC, 0x02][..]);
        let result = UVarint::decode(&mut buf, None);
        assert_eq!(300, result.unwrap().unwrap().0);

        let mut buf = BytesMut::from(&[0xE5, 0x8E, 0x26][..]);
        let result = UVarint::decode(&mut buf, None);
        assert_eq!(624485, result.unwrap().unwrap().0);
    }

    #[test]
    fn test_decode_leb128_incomplete() {
        // Incomplete: no terminating byte (MSB never 0)
        let mut buf = BytesMut::from(&[0x80][..]);
        let result = UVarint::decode(&mut buf, None);
        match result {
            Err(UVarintDecodeError::UnexpectedEndOfInput) =>  {},
            _ => panic!("Expected UnexpectedEndOfInput"),
        }
    }

    #[test]
    fn test_decode_leb128_overflow() {
        // Would overflow a u32 if interpreted literally
        let mut buf = BytesMut::from(&[0xFF, 0xFF, 0xFF, 0xFF, 0x0F][..]); // 0xFFFFFFFF
        assert_eq!(
            u32::MAX,
            UVarint::decode(&mut buf, None).unwrap().unwrap().0
        );

        // Actually overflowing (more than 5 bytes)
        let mut buf = BytesMut::from(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF][..]);
        let result = UVarint::decode(&mut buf, None);
        match result {
            Err(e) => match e {
                UVarintDecodeError::Overflow => {}
                _ => panic!("Expected Overflow, got {}", e),
            },
            _ => panic!("Expected Overflow, got Ok(_)"),
        }
    }

    #[test]
    fn test_encode_simple() {
        let u = UVarint(4);
        let mut buf = BytesMut::new();
        u.encode(&mut buf).unwrap();

        assert_eq!(4, *buf.iter().next().unwrap());
    }
}
