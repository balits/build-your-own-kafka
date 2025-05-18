#![allow(dead_code)]

use anyhow::Context;
use bytes::Bytes;
use message::{codec::KafkaCodec, headers::ResponseHeaderV0, response::KafkaResponse};
use tokio::net::TcpListener;
use tokio_util::codec::Framed;
use tracing::{info, warn};
use tracing_subscriber::FmtSubscriber;

mod message;
mod requests;

#[derive(Debug)]
struct Response {
    message_size: i32,
    header: ResponseHeader,
    body: Bytes,
}

#[derive(Debug)]
enum ResponseHeader {
    V0 { correlation_id: i32 },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .context("setting default subscriber failed")?;
    }

    let listener = TcpListener::bind("127.0.0.1:9092").await?;
    info!("Listening on port 9092");

    loop {
        let (socket, addr) = listener.accept().await.context("Accepting sockets")?;
        info!("Socket connected {addr}");
        let codec = KafkaCodec {};

        tokio::spawn(async move {
            use tokio_stream::StreamExt;
            use futures::SinkExt;
            let mut framed = Framed::new(socket, codec);

            while let Some(req) = framed.next().await {
                info!("Got some frame");
                match req {
                    Err(e) => warn!("Frame errored: {e}"),
                    Ok(req) => {
                        info!("Frame recieved {:?}", req);
                        let body = "hibaby";
                        let res = KafkaResponse {
                            message_size: body.len() as i32,
                            header: ResponseHeaderV0 {
                                correlation_id: req.header.correlation_id
                            },
                            body: body.into()
                        };

                        if let Err(e) =  framed.send(res).await {
                            warn!("Error replying to request: {:?}", e);
                        } else   {
                           info!("Response sent") 
                        }
                    }
                }
            }

            info!("Socket disconnected {addr}");
        });

    }

}
