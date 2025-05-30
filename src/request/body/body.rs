use anyhow::bail;
use bytes::BytesMut;

use crate::types::ApiKeys;
use crate::{
    codec::{Decoder, WireLen},
    unwrap_decode,
};

use super::api_versions_body::ApiVersionsRequestBody;
use super::describe_topic_partitions_body::DescribeTopicPartitionsBody;

#[derive(Debug)]
pub enum RequestBody {
    ApiVersions(ApiVersionsRequestBody),
    DescribeTopicPartitions(DescribeTopicPartitionsBody),
}

impl RequestBody {
    pub fn decode_by_key(
        key: &ApiKeys,
        src: &mut BytesMut,
        size: Option<usize>,
    ) -> anyhow::Result<Option<Self>> {
        match key {
            ApiKeys::ApiVersions => {
                let inner = unwrap_decode!(ApiVersionsRequestBody::decode(src, size));
                Ok(Some(RequestBody::ApiVersions(inner)))
            }
            ApiKeys::DescribeTopicPartitions => {
                let inner = unwrap_decode!(DescribeTopicPartitionsBody::decode(src, size));
                Ok(Some(RequestBody::DescribeTopicPartitions(inner)))
            }
            k => {
                bail!("Couldnt decode body based on api key {k} as it is unimplemented!")
            }
        }
    }
}

impl WireLen for RequestBody {
    fn wire_len(&self) -> usize {
        match self {
            RequestBody::ApiVersions(b) => b.wire_len(),
            RequestBody::DescribeTopicPartitions(b) => b.wire_len(),
        }
    }
}

