use anyhow::{self, bail};

use crate::{
    types::ApiKeys,
    primitives::CompactArray,
    request::KafkaRequest,
    response::{ApiVersion, ApiVersionResponseBody, KafkaResponse, ResponseBody, ResponseHeaderV0},
    WireLen,
};

pub fn handle_request(req: &KafkaRequest) -> anyhow::Result<KafkaResponse> {
    match req.header.request_api_key {
        ApiKeys::ApiVersions => handle_api_version(req),
        ApiKeys::DescribeTopicPartitions => handle_describe_topic_partition(req),
        _ => bail!("Api key not implemented"),
    }
}

fn handle_api_version(req: &KafkaRequest) -> anyhow::Result<KafkaResponse> {
    anyhow::ensure!(
        req.header.request_api_key == ApiKeys::ApiVersions,
        "request did not specify the ApiVersion apikey"
    );
    let header = ResponseHeaderV0::new(req.header.correlation_id);

    // "Assume that your broker only supports versions 0 to 4."
    let res = if req.header.request_api_version > 4 || req.header.request_api_version < 0 {
        let body = ResponseBody::ApiVersion(ApiVersionResponseBody {
            error_code: 35,
            api_versions: CompactArray::new(),
            throttle_time: 0,
            tag_buffer: CompactArray::new(),
        });
        let message_size = (header.wire_len() + body.wire_len()) as i32;
        KafkaResponse::new(message_size, header, body)
    } else {
        let mut api_versions = CompactArray::new();
        api_versions.push(ApiVersion::new(18, 0, 4));
        api_versions.push(ApiVersion::new(75, 0, 0));

        let body_inner = ApiVersionResponseBody::new(0, api_versions, 0);
        let body = ResponseBody::ApiVersion(body_inner);

        let message_size = (header.wire_len() + body.wire_len()) as i32;
        KafkaResponse::new(message_size, header, body)
    };

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
fn handle_describe_topic_partition(_req: &KafkaRequest) -> anyhow::Result<KafkaResponse> {
    todo!()
}
