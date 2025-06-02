use crate::codec::{Decoder, WireLen};
use crate::types::{TagBuf, TopicInRequest};
use crate::{primitives::*, unwrap_decode};
use anyhow;
use bytes::Buf;
use kafka_macros::WireLen;

#[derive(Debug, WireLen)]
pub struct DescribeTopicPartitionsRequestBody {
    pub topics: CompactArray<TopicInRequest>,
    pub partition_limit: i32,
    pub cursor: u8,
    tag_buffer: TagBuf,
}

impl DescribeTopicPartitionsRequestBody {
    pub fn new(topics: CompactArray<TopicInRequest>, partition_limit: i32) -> Self {
        Self {
            topics,
            partition_limit,
            cursor: 0xFF,
            tag_buffer: TagBuf::new(),
        }
    }
}

impl Decoder for DescribeTopicPartitionsRequestBody {
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
        let partition_limit = src.get_i32();
        let body = DescribeTopicPartitionsRequestBody::new(topics, partition_limit);

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
