pub struct ResponseHeaderV0 {
    pub(crate) correlation_id: i32,
}

impl ResponseHeaderV0 {
    fn new(correlation_id: i32) -> Self {
        Self { correlation_id }
    }
}

#[derive(Debug)]
pub struct RequestHeaderV2 {
    pub(crate) request_api_key: i16,
    pub(crate) request_api_version: i16,
    pub(crate) correlation_id: i32,
    pub(crate) client_id: String,
    // TODO Optional tagged fields
    // tag_buffer: Vec<u8>,
}

impl RequestHeaderV2 {
    pub fn new(
        request_api_key: i16,
        request_api_version: i16,
        correlation_id: i32,
        client_id: String,
    ) -> Self {
        Self {
            request_api_key,
            request_api_version,
            correlation_id,
            client_id,
        }
    }
}

// fn decode(mut bytes: &mut BytesMut) -> anyhow::Result<Self> {
//     let request_api_key = bytes.get_i16();
//     let request_api_version = bytes.get_i16();
//     let correlation_id = bytes.get_i32();
//     let client_id = decode_compact_string(&mut bytes);
//     let tag_buffer = {
//         match decode_varuint(&mut bytes) {
//             Some(len) => {
//                 if bytes.remaining() < len as usize {
//                     anyhow::bail!("Something went wrong with varuint")
//                 }
//                 bytes.advance(len as usize);
//                 Vec::with_capacity(len as usize)
//             }
//             _ => {
//                 // TODO maybe after that we should error out
//                 anyhow::bail!("Something went wrong with varuint")
//             }
//         }
//     };
//     Ok(Self {
//         request_api_key,
//         request_api_version,
//         correlation_id,
//         client_id,
//         tag_buffer,
//     })
// }
//
// fn encode(&self, buf: &mut BytesMut) -> anyhow::Result<()> {
//     buf.put_i16(self.request_api_key);
//     buf.put_i16(self.request_api_version);
//     buf.put_i32(self.correlation_id);
//     buf.put_slice(self.client_id.as_bytes()); // noop if len == 0
//     buf.put_slice(self.tag_buffer.as_slice()); // noop if len == 0
//
//     Ok(())
// }
//
// fn bufsize_hint(&self) -> usize {
//     8 + self.client_id.len() + self.tag_buffer.len()
// }
