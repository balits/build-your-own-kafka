use anyhow::{self, bail};

use crate::{
    WireLen,
    primitives::{ApiKeys, CompactArray},
    request::KafkaRequest,
    response::{ApiVersion, ApiVersionResponseBody, KafkaResponse, ResponseBody, ResponseHeaderV0},
};

pub fn handle_request(req: &KafkaRequest) -> anyhow::Result<KafkaResponse> {
    match req.header.request_api_key {
        ApiKeys::ApiVersions => handle_api_version(req),
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
