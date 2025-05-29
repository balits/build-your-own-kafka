mod lib;

pub use lib::{Encoder, Decoder, WireLen, MAX_MESSAGE_SIZE};

macro_rules! impl_wire_length {
    ( $( $t:ty )*) => {
        $(
            impl WireLen for $t {
                fn wire_len(&self) -> usize {
                    size_of::<$t>()
                }
            }
        )*
    };
}

impl_wire_length!(u8 u16 u32 u64 i8 i16 i32 i64 usize isize bool);

/// A convenience macro which makes it easier to compose `decode` functions
/// defined in both `tokio_util::codec::Decoder` and my custom Decoder.
/// Use it to return early in a decode function, or in any function which returns an
/// `Result<Option<T, E>>`, where T, E is unkown to this macro, so the user should be aware of
/// them, and call this macro in appropriate places
///
/// # Examples
/// 
/// ```rust
/// let x: CustomType = match CustomType::decode(src)
///     Ok(opt) => match opt {
///         Some(val) => val    
///         None => return Ok(None)
///     }
///     Err(e) => return Err(e)
/// };
/// ```
#[macro_export]
macro_rules! unwrap_decode {
    ( $expr: expr) => {
        (match $expr {
            Ok(Some(val)) => val,
            Ok(None) => return Ok(None),
            Err(e) => return Err(e),
        })
    };
}
