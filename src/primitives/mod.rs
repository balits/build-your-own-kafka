/// Upper limit on custom strings, like NULLABLE_STRING
/// or COMPACT_STRING
pub const MAX_STRING_SIZE: usize = 128;

pub(crate) mod compact_array;
pub(crate) mod compact_string;
pub(crate) mod tag;
pub(crate) mod uvarint;

pub(crate) use compact_array::CompactArray;
pub(crate) use compact_string::CompactString;
pub(crate) use tag::Tag;
pub(crate) use uvarint::UVarint;
