/// Upper limit on custom strings, like NULLABLE_STRING
/// or COMPACT_STRING, this is purely for my convenience
pub const MAX_STRING_SIZE: usize = 128;

mod api_keys;
mod compact_array;
mod compact_string;
mod nullable_string;
mod tag;
mod uvarint;

pub(crate) use api_keys::ApiKeys;
pub(crate) use compact_array::CompactArray;
pub(crate) use compact_string::CompactString;
pub(crate) use nullable_string::NullableString;
pub(crate) use tag::Tag;
pub(crate) use uvarint::UVarint;
