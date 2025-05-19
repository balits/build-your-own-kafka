use crate::message::{
    api_keys::ApiKeys, headers::ResponseHeaderV0, request::KafkaRequest, response::KafkaResponse,
};

pub struct RequestHandler {}
use anyhow::{self, bail};

impl Default for RequestHandler {
    fn default() -> Self {
        Self {}
    }
}

impl RequestHandler {
    // TODO refactor, so that if an error occurs in a handler,
    // we send back a `KafkaResponse` with some error field
    pub fn handle(&self, req: KafkaRequest) -> anyhow::Result<KafkaResponse> {
        match req.header.request_api_key {
            ApiKeys::ApiVersions => Self::handle_api_version(req),
            ApiKeys::UNIMPLEMENTED => bail!("Api key not implemented"),
        }
    }

    fn handle_api_version(req: KafkaRequest) -> anyhow::Result<KafkaResponse> {
        anyhow::ensure!(
            req.header.request_api_key == ApiKeys::ApiVersions,
            "request did not specify the ApiVersion apikey"
        );
        let body = [0x00, 0x23];
        let header = ResponseHeaderV0::new(req.header.correlation_id);

        // body is well within MAX_BODY_SIZE;
        KafkaResponse::from_raw_parts(header, &body)
    }
}
