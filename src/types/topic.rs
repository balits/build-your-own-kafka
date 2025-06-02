use bytes::BufMut;
use kafka_macros::{Encoder, WireLen};

use crate::{
    codec::{Decoder, Encoder},
    primitives::{Bool, CompactArray, CompactString},
    types::empty_tagbuf,
    unwrap_decode,
};

use super::TagBuf;

#[derive(Debug, WireLen)]
pub struct TopicInRequest {
    pub(crate) name: CompactString,
    tag_buffer: TagBuf,
}

impl TopicInRequest {
    pub fn new<S>(name: S) -> Self
    where
        S: Into<CompactString>,
    {
        Self {
            name: name.into(),
            tag_buffer: TagBuf::new(),
        }
    }
}

impl Decoder for TopicInRequest {
    fn decode(src: &mut bytes::BytesMut, _: Option<usize>) -> anyhow::Result<Option<Self>>
    where
        Self: Sized + crate::WireLen,
    {
        let name = unwrap_decode!(CompactString::decode(src, None));
        let _tag_buffer = unwrap_decode!(TagBuf::decode(src, None));

        Ok(Some(TopicInRequest::new(name)))
    }
}

#[derive(Debug, WireLen)]
pub struct TopicInResponse {
    error_code: u16,
    topic_name: CompactString,
    topic_id: Vec<u8>,
    is_internal: Bool,
    partitions: CompactArray<Partition>,
    topic_authorized_ops: u32,
    tag_buffer: TagBuf,
}

impl TopicInResponse {
    pub fn new(error_code: u16, topic_name: impl Into<CompactString>) -> Self {
        let topic_id = (0..32).map(|_| 0).collect();
        Self {
            error_code,
            topic_name: topic_name.into(),
            topic_id,
            is_internal: Bool::False,
            partitions: CompactArray::new(),
            topic_authorized_ops: 0,
            tag_buffer: empty_tagbuf(),
        }
    }
}

impl Encoder for TopicInResponse {
    fn encode(&self, dest: &mut bytes::BytesMut) -> anyhow::Result<()> {
        dest.put_u16(self.error_code);
        self.topic_name.encode(dest)?;
        for i in 0..16 {
            dest.put_u8(self.topic_id[i]);
        }
        self.is_internal.encode(dest)?;
        self.partitions.encode(dest)?;
        dest.put_u32(self.topic_authorized_ops);
        self.tag_buffer.encode(dest)?;
        Ok(())
    }
}

#[derive(Debug, WireLen, Encoder)]
pub struct Partition {
    error_code: u16,
    partition_index: u32,
    leader_id: u32,
    leader_epoch: u32,
    replicas: CompactArray<ReplicaNode>,
    in_sync_replicas: CompactArray<ReplicaNode>,
    eligble_leader_replicas: CompactArray<ReplicaNode>,
    offline_replicas: CompactArray<ReplicaNode>,
    tag_buffer: TagBuf,
}

// impl Encoder for Partition {
//     fn encode(&self, dest: &mut bytes::BytesMut) -> anyhow::Result<()> {
//         dest.put_u16(self.error_code);
//         dest.put_u32(self.partition_index);
//         dest.put_u32(self.leader_id);
//         dest.put_u32(self.leader_epoch);
//         self.replicas.encode(dest)?;
//         self.in_sync_replicas.encode(dest)?;
//         self.eligble_leader_replicas.encode(dest)?;
//         self.offline_replicas.encode(dest)?;
//         self.tag_buffer.encode(dest)?;
//         Ok(())
//     }
// }

#[derive(Debug, WireLen, Encoder)]
pub struct ReplicaNode(u32);
