use anyhow::{self, bail};
use tracing::debug;

use crate::{
    WireLen,
    primitives::CompactArray,
    request::{KafkaRequest, RequestBody},
    response::{
        KafkaResponse, ResponseHeaderV0,
        body::{ApiVersionsResponseBody, DescribeTopicPartitionsResponseBody, ResponseBody},
    },
    types::{ApiKeys, ApiVersion, TopicInResponse},
};

pub fn handle_request(req: &KafkaRequest) -> anyhow::Result<KafkaResponse> {
    match req.header.request_api_key {
        ApiKeys::ApiVersions => handle_api_version(req),
        ApiKeys::DescribeTopicPartitions => handle_describe_topic_partition(req),
        _ => bail!("api key not implemented"),
    }
}

fn handle_api_version(req: &KafkaRequest) -> anyhow::Result<KafkaResponse> {
    anyhow::ensure!(
        req.header.request_api_key == ApiKeys::ApiVersions,
        "request did not specify the ApiVersion apikey"
    );
    anyhow::ensure!(
        matches!(req.body, RequestBody::ApiVersions(_)),
        "Expected api_versions body in api_verions request"
    );

    let header = ResponseHeaderV0::new(req.header.correlation_id);

    // "Assume that your broker only supports versions 0 to 4."
    let body = if req.header.request_api_version > 4 || req.header.request_api_version < 0 {
        ResponseBody::ApiVersions(ApiVersionsResponseBody::new(35, CompactArray::new(), 0))
    } else {
        let mut api_versions = CompactArray::with_capacity(2);
        api_versions.push(ApiVersion::new(18, 0, 4));
        api_versions.push(ApiVersion::new(75, 0, 0));

        let body_inner = ApiVersionsResponseBody::new(0, api_versions, 0);
        ResponseBody::ApiVersions(body_inner)
    };
    let message_size = (header.wire_len() + body.wire_len()) as i32;
    let res = KafkaResponse::new(message_size, header, body);

    Ok(res)
}

/// It'll then connect to your server on port 9092 and send a DescribeTopicPartitions (v0) request. The request will contain a single topic with 1 partition.
///
/// ## The tester will validate that:
///
/// - The first 4 bytes of your response (the "message length") are valid.
/// - The correlation ID in the response header matches the correlation ID in the request header.
/// - The error code in the response body is 3 (UNKNOWN_TOPIC_OR_PARTITION).
/// - The response body should be valid DescribeTopicPartitions (v0) Response.
/// - The topic_name field in the response should be equal to the topic name sent in the request.
/// - The topic_id field in the response should be equal to 00000000-0000-0000-0000-000000000000.
/// - The partitions field in the response should be empty. (As there are no partitions assigned to this non-existent topic.)
fn handle_describe_topic_partition(req: &KafkaRequest) -> anyhow::Result<KafkaResponse> {
    anyhow::ensure!(
        req.header.request_api_key == ApiKeys::DescribeTopicPartitions,
        "request did not specify the DescribeTopicPartitions apikey"
    );
    let header = ResponseHeaderV0::respond(req);
    debug!(header = ?header);
    let mut topics = CompactArray::new();
    let RequestBody::DescribeTopicPartitions(ref reqbody) = req.body else {
        bail!("Invalid request body for DescribeTopicPartitions")
    };
    debug!(reqbody= ?reqbody);

    // next for loop takes care of it, but i still want to signal if this happends for now
    anyhow::ensure!(reqbody.topics.len() == 1, "Multiple topics were sent");

    for t in reqbody.topics.iter() {
        topics.push(TopicInResponse::new(0, t.name.clone()));
    }

    let body_inner = DescribeTopicPartitionsResponseBody::new(0, topics, 0, 0);
    let body = ResponseBody::DescribeTopicPartitions(body_inner);

    let message_size = (header.wire_len() + body.wire_len()) as i32;
    let res = KafkaResponse::new(message_size, header, body);

    dbg!(&req);
    dbg!(&res);
    Ok(res)
}
