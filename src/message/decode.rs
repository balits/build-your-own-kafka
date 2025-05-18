use anyhow::{self, ensure, Context};
use bytes::{Buf, BytesMut};
use tracing::info;
use super::codec::MAX_CLIENT_ID_SIZE;

/// Decoding a nullable string from a bytes source
/// 
/// This functions is meant to be called inside tokio_util::codec::Decoder::decode
/// as it reserves additional bytes if the buffer is not big enough,
/// and returns Ok(None) in such cases
pub fn decode_nullable_string(buf: &mut BytesMut) -> Result<Option<String>, anyhow::Error> {
    if buf.len() < 2 {
        // not enough space for length,
        // reserve and signal to the callers that the operation failed
        // but no error happend, so we can wait on the next frame
        buf.reserve(2);
        return Ok(None)
    }

    let len = buf.get_i16();

    if len == -1 {
        return Ok(Some(String::from("")));
    } 

    if len == 0 {
        return Ok(Some(String::new()));
    }
 
    ensure!(len > 0 , "Negative ammount of INT16");
    ensure!((len as usize) < MAX_CLIENT_ID_SIZE, "client id length is bigger than allowed {MAX_CLIENT_ID_SIZE}");

    let len = len as usize;
    info!("\tNullable: len {len}");

    if buf.len() < len {
        buf.reserve(len);
        return Ok(None)
        // bail!(
        //     "Buffer doesnt have enough bytes to read bytes to string (expected {}, got {})",
        //     len,
        //     buf.len()
        // )
    }

    let data = buf.split_to(len);
    info!("Nullable: raw data: {:?}", data);

    Ok(Some(
        String::from_utf8(data.to_vec())
            .context("Parsing nullable string with len {len}")?
    ))
}
