use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9092").await.unwrap();

    let payload = [
        0x00, 0x00, 0x00, 0x0D, // message_size = 13
        0x00, 0x12, // request_api_key = 18
        0x00, 0x02, // request_api_version = 2
        0x00, 0x00, 0x00, 0x2A, // correlation_id = 42
        0x00, 0x03, // client_id length = 3
        0x66, 0x6F, 0x6F, // "foo"
    ];

    stream.write_all(&payload).await.unwrap();
    let mut buf = vec![0; 14];
    stream.read_exact(&mut buf).await.unwrap();
    println!("{:02X?}", buf);

    let strlen = i32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);

    println!("{}", std::str::from_utf8(&buf[5..]).unwrap());
}
