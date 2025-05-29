use bytes::BytesMut;

pub trait Decoder {
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

/// Custom Decoder trait to turn structs into bytes.
/// the `dest` buffer could be passed to other calls to `encode`
/// provided that they grow the buffer to the appropriate size.
/// Alternatively a call to the top level `encode`, such when encoding
/// a `KafkaResponse` its optimal to reserve space using the `WireLen` trait
pub trait Encoder{
    fn encode(&self, dest: &mut BytesMut) ->  anyhow::Result<()>;
}
