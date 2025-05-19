use crate::message::api_keys::ApiKeys;

pub struct ResponseHeaderV0 {
    pub(crate) correlation_id: i32,
}

impl ResponseHeaderV0 {
    pub fn new(correlation_id: i32) -> Self {
        Self { correlation_id }
    }

    pub fn size(&self) -> usize {
        size_of_val(self)
    }
}

#[derive(Debug)]
pub struct RequestHeaderV2 {
    pub(crate) request_api_key: ApiKeys,
    #[allow(dead_code)]
    pub(crate) request_api_version: i16,
    pub(crate) correlation_id: i32,
    pub(crate) client_id: String,
    // TODO Optional tagged fields
    // tag_buffer: Vec<u8>,
}

impl RequestHeaderV2 {
    /// Safety
    ///
    /// this constructor does nothing to
    /// check if the api_version is anything meaningfull.
    pub fn new(
        request_api_key: i16,
        request_api_version: i16,
        correlation_id: i32,
        client_id: String,
    ) -> Self {
        Self {
            request_api_key: request_api_key.into(),
            request_api_version,
            correlation_id,
            client_id,
        }
    }

    /// This returns the length of the header
    /// in bytes, using the strings length, rather than its
    /// stack memory representation (ptr, len, cap)
    #[inline]
    pub fn size(&self) -> usize {
        2 + 2 + 4 + 2 + self.client_id.len()
    }
}
