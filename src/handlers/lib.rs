use crate::{message::{
    api_keys::ApiKeys, headers::ResponseHeaderV0, request::KafkaRequest, response::{ApiVersion, ApiVersionResponseBody, KafkaResponse, ResponseBody},
}, primitives::CompactArray, WireLen};

use anyhow::{self, bail};


pub fn handle_request(req: &KafkaRequest) -> anyhow::Result<KafkaResponse> {
    match req.header.request_api_key {
        ApiKeys::ApiVersions => handle_api_version(req),
        ApiKeys::UNIMPLEMENTED => bail!("Api key not implemented"),
    }
}

fn handle_api_version(req: &KafkaRequest) -> anyhow::Result<KafkaResponse> {
    anyhow::ensure!(
        req.header.request_api_key == ApiKeys::ApiVersions,
        "request did not specify the ApiVersion apikey"
    );
    let header = ResponseHeaderV0::new(req.header.correlation_id);

    let mut api_versions = CompactArray::new();
    api_versions.push(ApiVersion::new(18,4,4));

    let body_inner = ApiVersionResponseBody::new(0, api_versions, 0);
    let body = ResponseBody::ApiVersion(body_inner);

    let message_size = (header.wire_len() + body.wire_len()) as i32;
    let res = KafkaResponse::new(message_size, header, body);

    Ok(res)
}
