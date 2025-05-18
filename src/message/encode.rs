use anyhow;
use bytes::BytesMut;

pub trait Encode {
    fn encode(&self, buf: &mut BytesMut) -> anyhow::Result<()>;
    fn bufsize_hint(&self) -> usize;
}
