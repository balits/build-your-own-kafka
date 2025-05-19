use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9092").await.unwrap();
    let payload = [
        0x00, 0x00, 0x00, 0x10, // message_size = 16
        0x00, 0x12, // api_key = 18
        0x00, 0x02, // api_version = 2
        0x00, 0x00, 0x00, 0x2A, // correlation_id = 42
        0x00, 0x03, // client_id length = 3
        0x66, 0x6F, 0x6F, // "foo" as client_id
        0x66, 0x6F, 0x6F, // "foo" as body
    ];

    stream.write_all(&payload).await.unwrap();
}
