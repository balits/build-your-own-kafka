use bytes::BytesMut;
use kafka::primitives::CompactString;
use kafka::{Encoder, WireLen};
use kafka_macros::{Encoder, WireLen};

#[derive(Debug, Encoder, WireLen)]
struct MyStruct {
    int: i32,
    byte: u8,
    compact_string: CompactString,
}

fn main() -> std::io::Result<()> {
    let s = MyStruct {
        int: 10,
        byte: 30,
        compact_string: CompactString::from("hi"),
    };
    let mut buf = BytesMut::with_capacity(s.wire_len());
    s.encode(&mut buf).expect("problem");
    println!("{:x?}", buf.iter().as_slice());
    Ok(())
}
