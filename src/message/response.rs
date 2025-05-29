use bytes::{BufMut, BytesMut};
use kafka_macros::WireLen;

use crate::{codec::lib::Encoder, message::headers::ResponseHeaderV0, primitives::{CompactArray, Tag}, WireLen};

#[derive(Debug, WireLen)]
pub struct KafkaResponse {
    pub(crate) message_size: i32,
    pub(crate) header: ResponseHeaderV0,
    pub(crate) body: ResponseBody,
}

impl KafkaResponse {
    pub fn new(message_size: i32, header: ResponseHeaderV0, body: ResponseBody) -> Self {
        Self {message_size, header, body}
    }
}


#[derive(Debug)]
pub enum ResponseBody {
    ApiVersion(ApiVersionResponseBody)
}

impl WireLen for ResponseBody {
    fn wire_len(&self) -> usize {
        match self {
            ResponseBody::ApiVersion(body) => body.wire_len()
        }
    }
}

impl Encoder for ResponseBody {
    fn encode(&self, dest: &mut BytesMut) ->  anyhow::Result<()> {
        match self {
            ResponseBody::ApiVersion(body) => body.encode(dest)
        }
    }
}

#[derive(Debug, WireLen)]
pub struct ApiVersionResponseBody {
    error_code: u16,
    api_versions: CompactArray<ApiVersion>,
    throttle_time: u32,
    tag_buffer: CompactArray<Tag>
}

impl ApiVersionResponseBody {
    pub fn new(error_code: u16, api_versions: CompactArray<ApiVersion>, throttle_time: u32) -> Self {
        Self { error_code, api_versions, throttle_time, tag_buffer: CompactArray::new() }
    }
}

impl Encoder for ApiVersionResponseBody {
    fn encode(&self, dest: &mut BytesMut) ->  anyhow::Result<()> {
        dest.put_u16(self.error_code);
        self.api_versions.encode(dest)?;
        dest.put_u32(self.throttle_time);
        self.tag_buffer.encode(dest)?;
        Ok(())
    }
}

impl Encoder for KafkaResponse {
    /// This is the top level call to decode
    fn encode(&self, dest: &mut BytesMut) -> anyhow::Result<()> {
        dest.put_i32(self.message_size);
        dest.put_i32(self.header.correlation_id);

        let ResponseBody::ApiVersion(ref body) = self.body;
        dest.put_u16(body.error_code);
        dest.put_u8(body.api_versions.len() as u8 + 1); // since its N + 1 for N elements
        
        for ver in body.api_versions.iter() {
            dest.put_u16(ver.api_key);
            dest.put_u16(ver.min_version);
            dest.put_u16(ver.max_version);
            dest.put_u8(0); // tag buff
        }
        dest.put_u32(body.throttle_time);
        dest.put_u8(0); // tag buff

        Ok(())
    }
}

#[derive(Debug, WireLen)]
pub struct ApiVersion {
    pub(crate) api_key: u16,
    pub(crate) min_version: u16,
    pub(crate) max_version: u16,
    pub(crate) tag_buffer: CompactArray<Tag>
}

impl ApiVersion {
    pub fn new(api_key: u16, min_sup_version: u16, max_sup_version: u16) -> Self {
        Self { api_key, min_version: min_sup_version, max_version: max_sup_version, tag_buffer: CompactArray::new() }
    }
}

impl Encoder for ApiVersion {
    fn encode(&self, dest: &mut BytesMut) ->  anyhow::Result<()> {
       dest.put_u16(self.api_key);
       dest.put_u16(self.min_version);
       dest.put_u16(self.max_version);
       self.tag_buffer.encode(dest)
    }
}
 

#[cfg(test)]
mod tests {
    use super::super::super::primitives::*;
    use super::*;
    use bytes::Buf;


    #[test]
    fn test_response_simple_encode() {
        let corr_id = 20;
        let header = ResponseHeaderV0::new(corr_id);
        let mut api_versions = CompactArray::new();
        api_versions.push(ApiVersion::new(18,0,4));

        let body = ResponseBody::ApiVersion(ApiVersionResponseBody::new(0, api_versions, 0));
        let message_size = (header.wire_len() + body.wire_len()) as i32;
        let res = KafkaResponse::new(message_size, header, body);

        let mut buf = BytesMut::with_capacity(res.wire_len());
        res.encode(&mut buf).unwrap();

        assert_eq!(res.wire_len(), buf.len());
        
        let s = buf.iter().as_slice().to_vec();
        let c = buf.freeze();
        let mut buf = c.clone();

        assert_eq!(message_size, buf.get_i32()); // message_size 4 byte

        assert_eq!(corr_id, buf.get_i32()); // corr_id 4 byte

        assert_eq!(0, buf.get_u16()); // error code
        assert_eq!(1, buf.get_u8());   // len
                                       //

        assert_eq!(18, buf.get_u16());   // api key
        assert_eq!(0, buf.get_u16());   // min version
        assert_eq!(4, buf.get_u16());  // max version
        assert_eq!(0, buf.get_u8());   // tag buf len

        assert_eq!(0u32, buf.get_u32()); // throttle time 4 byte
        
        assert_eq!(0, buf.get_u8()); // tag buf len
        assert!(!buf.has_remaining()); 

        println!("Response body len: {}", buf.len());
        println!("{:X?}", &s[..4]);
        println!("{:X?}", &s[4..8]);
        println!("{:X?}", &s[8..9]);
        println!();
        println!("{:X?}", &s[9..11]);
        println!("{:X?}", &s[11..13]);
        println!("{:X?}", &s[13..15]);
        println!("{:X?}", &s[16]);
    }
}
