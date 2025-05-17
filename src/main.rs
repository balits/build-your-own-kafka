#![allow(dead_code)]

use anyhow::Context;
use bytes::{Buf, BufMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
struct KafkaResponse {
    message_size: i32,
    header: ResponseHeader,
}

#[derive(Debug)]
enum ResponseHeader {
    V0 { correlation_id: i32 },
}

impl KafkaResponse {
    fn new_response_v0(message_size: i32, correlation_id: i32) -> Self {
        Self {
            message_size,
            header: ResponseHeader::V0 { correlation_id },
        }
    }

    #[allow(unreachable_patterns)]
    fn to_bytes(&self) -> bytes::BytesMut {
        let mut buf = bytes::BytesMut::new();
        buf.put_i32(self.message_size);
        match self.header {
            ResponseHeader::V0 { correlation_id } => buf.put_i32(correlation_id),
            _ => unimplemented!(),
        }
        buf
    }
}

#[derive(Debug)]
struct KafkaRequest {
    message_size: i32,
    header: RequestHeader,
}

#[derive(Debug)]
enum RequestHeader {
    V2 {
        request_api_key: i16,
        request_api_version: i16,
        correlation_id: i32,
        client_id: String,
        tag_buffer: Vec<u8>, // Optional tagged fields
    },
}

impl KafkaRequest {
    #[allow(unreachable_patterns)]
    fn to_bytes(self) -> bytes::BytesMut {
        let mut buf = bytes::BytesMut::new();
        buf.put_i32(self.message_size);
        match self.header {
            RequestHeader::V2 {
                request_api_key,
                request_api_version,
                correlation_id,
                client_id,
                tag_buffer,
            } => {
                buf.put_i16(request_api_key);
                buf.put_i16(request_api_version);
                buf.put_i32(correlation_id);
                buf.put_slice(client_id.as_bytes()); // noop if len == 0
                buf.put_slice(&tag_buffer); // noop if len == 0
            }
            _ => unimplemented!(),
        }
        buf
    }

    fn new_v2_from_buf(buf: &mut bytes::BytesMut) -> Self {
        Self {
            message_size: buf.get_i32(),
            header: RequestHeader::V2 {
                request_api_key: buf.get_i16(),
                request_api_version: buf.get_i16(),
                correlation_id: buf.get_i32(),
                client_id: {
                    let len = buf.get_i16();
                    if len == -1 {
                        String::from("")
                    } else {
                        let raw = buf.copy_to_bytes(len as usize);
                        String::from_utf8_lossy(&raw).to_string()
                    }
                },
                tag_buffer: vec![],
            },
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9092").await?;

    async fn handle(mut socket: TcpStream) -> anyhow::Result<()> {
        let mut buf = bytes::BytesMut::with_capacity(1024);
        match socket.read_buf(&mut buf).await {
            Ok(0) => {
                anyhow::bail!("Connection closed by peer.");
            }
            Ok(n) => {
                println!("Received: {:?}", &buf[..n]);
                let req = KafkaRequest::new_v2_from_buf(&mut buf);
                dbg!(&req);
                let cid = match req.header {
                    RequestHeader::V2 { correlation_id, .. } => correlation_id,
                };
                let res = KafkaResponse::new_response_v0(0, cid);
                dbg!(&res);

                socket
                    .write_all(&res.to_bytes())
                    .await
                    .with_context(|| format!("Writing response {:?}", &res))?;
                socket
                    .flush()
                    .await
                    .with_context(|| format!("Writing response {:?}", &res))?;
            }
            Err(e) => {
                anyhow::bail!("Read error: {:?}", e);
            }
        }
        Ok(())
    }

    loop {
        let (socket, _addr) = listener.accept().await.context("Accepting sockets")?;

        tokio::spawn(async move {
            if let Err(e) = handle(socket).await {
                eprintln!("{}", e)
            }
        });
    }
}
