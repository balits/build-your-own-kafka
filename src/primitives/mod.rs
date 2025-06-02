//! Types that are deemed primitive and composable by the kafka docs
//! These are the building blocks of more custom types

/// Upper limit on custom strings, like NULLABLE_STRING
/// or COMPACT_STRING, this is purely for my convenience
pub const MAX_STRING_SIZE: usize = 128;

mod bool;
mod compact_array;
mod compact_string;
mod nullable_string;
mod uvarint;

pub use bool::Bool;
pub use compact_array::CompactArray;
pub use compact_string::CompactString;
pub use nullable_string::NullableString;
pub use uvarint::UVarint;
