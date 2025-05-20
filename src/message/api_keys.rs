use std::fmt::Display;

/// This enum is not a one to one port of the original Kafka
/// enum. As I wont be implementing every api request,
/// I reserved 0 as the Unimplemented request, which makes it
/// easier for me to code as I dont have to wrap everything into
/// Options and Results or use monadic functions.
#[derive(Debug, PartialEq, Eq)]
pub enum ApiKeys {
    ApiVersions = 18,
    UNIMPLEMENTED = 0,
}

impl From<i16> for ApiKeys {
    fn from(value: i16) -> Self {
        match value {
            18 => ApiKeys::ApiVersions,
            _ => ApiKeys::UNIMPLEMENTED,
        }
    }
}

impl Display for ApiKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
