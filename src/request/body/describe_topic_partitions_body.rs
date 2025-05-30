use kafka_macros::WireLen;
use crate::types::{TagBuf, Topic};
use crate::{primitives::*, unwrap_decode};
use crate::codec::{Decoder, WireLen};
use anyhow;
use bytes::Buf;


#[derive(Debug, WireLen)]
pub struct DescribeTopicPartitionsBody {
    topics: CompactArray<Topic>,
    partition_limit: u32,
    cursor: u8,
    tag_buffer: TagBuf
}

impl DescribeTopicPartitionsBody {
    pub fn new(topics: CompactArray<Topic>, partition_limit: u32) -> Self {
        Self {
            topics, partition_limit, cursor: 0xFF, tag_buffer:TagBuf::new()
        }
    }
}

impl Decoder for DescribeTopicPartitionsBody {
    fn decode(src: &mut bytes::BytesMut, size: Option<usize>) -> anyhow::Result<Option<Self>>
    where
        Self: Sized + WireLen,
    {
        if let Some(sz) = size {
            if src.remaining() < sz {
                src.reserve(sz);
                return Ok(None);
            }
        }
        let topics = unwrap_decode!(CompactArray::decode(src, None));
        if src.remaining() < 4 {
            src.reserve(4);
            return Ok(None);
        }
        let partition_limit = src.get_u32();
        let body = DescribeTopicPartitionsBody::new(topics, partition_limit);


        if let Some(sz) = size {
            let wl = body.wire_len();
            anyhow::ensure!(
                sz == wl,
                "Size of body does not meet expectations, got: {wl}, expected: {sz}"
            );
        }

        Ok(Some(body))
    }
}

