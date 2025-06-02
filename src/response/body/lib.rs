use crate::codec::{Encoder, WireLen};
use bytes::BytesMut;

use super::{
    ApiVersionsResponseBody, describe_topic_partitions::DescribeTopicPartitionsResponseBody,
};

#[derive(Debug)]
pub enum ResponseBody {
    ApiVersions(ApiVersionsResponseBody),
    DescribeTopicPartitions(DescribeTopicPartitionsResponseBody),
}

impl WireLen for ResponseBody {
    fn wire_len(&self) -> usize {
        match self {
            ResponseBody::ApiVersions(body) => body.wire_len(),
            ResponseBody::DescribeTopicPartitions(body) => body.wire_len(),
        }
    }
}

impl Encoder for ResponseBody {
    fn encode(&self, dest: &mut BytesMut) -> anyhow::Result<()> {
        match self {
            ResponseBody::ApiVersions(body) => body.encode(dest),
            ResponseBody::DescribeTopicPartitions(body) => body.encode(dest),
        }
    }
}
