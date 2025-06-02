use kafka_macros::{Encoder, WireLen};

use crate::{
    primitives::CompactArray,
    types::{TagBuf, TopicInResponse},
};

#[derive(Debug, WireLen, Encoder)]
pub struct DescribeTopicPartitionsResponseBody {
    pub throttle_time: i32,
    pub topics: CompactArray<TopicInResponse>,
    pub partition_limit: i32,
    pub cursor: u8,
    tag_buffer: TagBuf,
}

impl DescribeTopicPartitionsResponseBody {
    pub fn new(
        throttle_time: i32,
        topics: CompactArray<TopicInResponse>,
        partition_limit: i32,
        cursor: u8,
    ) -> Self {
        Self {
            throttle_time,
            topics,
            partition_limit,
            cursor,
            tag_buffer: TagBuf::new(),
        }
    }
}

// impl Encoder for DescribeTopicPartitionsResponseBody {
//     fn encode(&self, dest: &mut bytes::BytesMut) -> anyhow::Result<()> {
//         dest.put_i32(self.throttle_time);
//         self.topics.encode(dest)?;
//         dest.put_i32(self.partition_limit);
//         dest.put_u8(self.cursor);
//         self.tag_buffer.encode(dest)?;
//         Ok(())
//     }
// }

// #[derive(Debug, WireLen)]
// struct Cursor {
//     pub topic_name: CompactString,
//     pub partition_index: i32,
//     pub tag_buffer: TagBuf,
// }
//
// impl Cursor {
//     pub fn new<C: Into<CompactString>>(topic_name: C, partition_index: i32) -> Self {
//         Self {
//             topic_name: topic_name.into(),
//             partition_index,
//             tag_buffer: TagBuf::new(),
//         }
//     }
// }
