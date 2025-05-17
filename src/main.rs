use anyhow::Context;
use bytes::BufMut;
use tokio::{io::AsyncWriteExt, net::TcpListener};

#[derive(Debug)]
struct Response {
    message_size: i32,
    header: ResponseHeader,
}

#[derive(Debug)]
enum ResponseHeader {
    HeaderV0 { correlation_id: i32 },
}

impl Response {
    fn new_v0(message_size: i32, correlation_id: i32) -> Self {
        Self {
            message_size,
            header: ResponseHeader::HeaderV0 { correlation_id },
        }
    }

    fn to_bytes(self) -> bytes::BytesMut {
        let mut buf = bytes::BytesMut::new();
        buf.put_i32(self.message_size);
        match self.header {
            ResponseHeader::HeaderV0 { correlation_id } => buf.put_i32(correlation_id),
            _ => unimplemented!(),
        }
        buf
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9092").await?;

    loop {
        match listener.accept().await {
            Ok((mut sock, _addr)) => {
                sock.writable()
                    .await
                    .context("Waiting for socket to be writeable")?;

                let res = Response::new_v0(0, 7);

                sock.write_all(&res.to_bytes())
                    .await
                    .context("Writing response to socket")?;
            }
            Err(e) => println!("error: {}", e),
        }
    }

    Ok(())
}
