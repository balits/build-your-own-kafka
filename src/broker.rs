#![deny(clippy::pedantic)]
use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, thread::panicking};

use anyhow::{bail, Context};
use bytes::BytesMut;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::{debug, error, info, warn};

use crate::{
    WireLen,
    codec::{Decoder, Encoder},
    handlers::handle_request,
    request::KafkaRequest,
};

pub struct Broker {
    listener: TcpListener,
}

impl Broker {
    pub const ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9092);
    pub const READ_SIZE: usize = 128;

    pub async fn new() -> io::Result<Self> {
        Ok(Self {
            listener: TcpListener::bind(Self::ADDR).await?,
        })
    }

    async fn handle_socket(stream: TcpStream, addr: SocketAddr) -> anyhow::Result<()> {
        info!("{addr} connected");
        let (mut r, mut w) = tokio::io::split(stream);
        loop {
            let mut buf = BytesMut::with_capacity(Self::READ_SIZE); // <- TODO: handle bigger input sized or frames/dynamic reading
            let n = match r.read_buf(&mut buf).await {
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    warn!("Reading socket would block, advancing");
                    continue;
                }
                Err(e) => {
                    let err = format!("Error reading from socket: {}", e);
                    error!(err);
                    bail!(err);
                }
                Ok(0) => {
                    info!("{addr} disconnected");
                    return Ok(());
                }
                Ok(n) => n,
            };

            let req = KafkaRequest::decode(&mut buf.split_to(n), None);
            
            let req = req.context("Could not decode buffer #1 Result")?.context("Could not decode buffer #2 Option")?;
            debug!("request decoded: {:?}", req);
            let res = handle_request(&req).context("Handling request")?;
            debug!("request handled, generated response: {:?}", res);
            let mut buf = BytesMut::with_capacity(res.wire_len());
            res.encode(&mut buf)
                .context("Encoding response to buffer")?;
            w.write_all_buf(&mut buf)
                .await
                .context("Sending response buffer")?;
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        info!("Listening on {}", Self::ADDR);
        loop {
            let (stream, addr) = self
                .listener
                .accept()
                .await
                .context("Accepting new connection")?;

            tokio::spawn(async move {
                if let Err(e) = Self::handle_socket(stream, addr).await {
                    eprintln!("Error on socket's event loop");
                    for (i, cause) in e.chain().enumerate() {
                        eprintln!("\t{i}. {}", cause);
                    }
                }
            });
        }
    }
}

impl Drop for Broker {
    fn drop(&mut self) {
        info!("Dropping broker");
    }
}
