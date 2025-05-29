use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::Context;
use bytes::BytesMut;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use tracing::{error, info, debug, warn};

use crate::{codec::{Decoder, Encoder}, handlers::handle_request, request::KafkaRequest, WireLen};

pub struct Broker {
    listener: TcpListener
}

impl Broker {
    pub const ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9092);
    pub const READ_SIZE: usize = 128;

    pub async fn new() -> Self {
        Self {
            listener: TcpListener::bind(Self::ADDR).await.unwrap()
        }
    }

    async fn handle_socket(stream: TcpStream, addr: SocketAddr) -> anyhow::Result<()> {
        info!("{addr} connected");
        let  (mut r, mut w) = tokio::io::split(stream);
        loop {
            let mut buf = BytesMut::with_capacity(Self::READ_SIZE); // <- TODO: handle bigger input sized or frames/dynamic reading
            let n = match r.read_buf(&mut buf).await {
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    warn!("Reading socket would block, advancing");
                    continue;
                }
                Err(e) => {
                    error!("Error reading from socket: {}", e);
                    break;
                }
                Ok(0) => {
                    info!("{addr} disconnected");
                    return Ok(())
                }
                Ok(n) => n
            };

            let req = KafkaRequest::decode(&mut buf.split_to(n), None).context("Decoding buffer into request")?;
            if req.is_none() {
                error!("Could not decode buffer");
                continue;
            }
            let req = req.unwrap();

            let res = handle_request(&req).context("Handling request")?;
            let mut buf = BytesMut::with_capacity(res.wire_len());
            res.encode(&mut buf).context("Encoding response to buffer")?;
            debug!("{:X?}", &buf[..]);
            w.write_all_buf(&mut buf).await.context("Sending response buffer")?;
        }
        Ok(())
    }

    pub async fn run(self) -> anyhow::Result<()> {
        info!("Listening on {}", Self::ADDR);

        loop {
            let (stream, addr) = self.listener.accept().await.context("Accepting new connection")?;
            tokio::spawn(Self::handle_socket(stream, addr));
        }
    }
}

impl Drop for Broker {
    fn drop(&mut self) {
        info!("Dropping broker");
    }
}