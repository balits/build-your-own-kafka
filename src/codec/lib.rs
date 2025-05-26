/// Custom Decoder trait for structs,
/// meant to be used inside `tokio_util::coder::Decoder`
/// by nested structs
pub trait CustomDecoder {
    /// Error to be propagated to `tokio_util::codec::Decoder`
    type Error;

    /// Decodes bytes read from `src` as a new instance of Self.
    /// Inside of `tokio_utils::codec::Decoder` the buffer `src`
    /// is kept between each read. If `src` doesnt have enough
    /// size to decode into Self, it is advised to reserve some more
    /// space inside. `size_hint` is a way for the user to supply
    /// a size to reserve on a failed read attempt in cases
    /// where the size might be known beforehand.
    fn decode(
        src: &mut bytes::BytesMut,
        size_hint: Option<usize>,
    ) -> Result<Option<Self>, Self::Error>
    where
        Self: Sized + WireLen;
}

/// Returns the number of bytes
/// self takes up during encoding and decoding
/// with traits in `tokio_util::codec`
pub trait WireLen {
    fn wire_len(&self) -> usize;
}
