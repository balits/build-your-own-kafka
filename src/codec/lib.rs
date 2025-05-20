/// Returns the number of bytes
/// self takes up during encoding and decoding
/// with traits in `tokio_util::codec`
pub trait WireLength {
    fn wire_len(&self) -> usize;
}

/// Custom Decoder trait for structs,
/// meant to be used inside `tokio_util::coder::Decoder`
/// by nested structs
pub trait CustomDecoder {
    /// Error to be propagated to `tokio_util::codec::Decoder`
    type Error;

    fn decode(src: &mut bytes::BytesMut, size: usize) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized + WireLength;
}
