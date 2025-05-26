use kafka::codec::KafkaCodec;
use kafka::handlers::request_handler::RequestHandler;

use anyhow::Context;
use tokio::net::TcpListener;
use tokio_util::codec::Framed;
use tracing::{error, info, warn};
use tracing_subscriber::FmtSubscriber;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(tracing::Level::TRACE)
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .context("setting default subscriber failed")?;
    }

    let listener = TcpListener::bind("127.0.0.1:9092").await?;
    info!("Listening on port 9092");

    loop {
        let (socket, addr) = listener.accept().await.context("Accepting sockets")?;
        info!("Socket connected {addr}");
        let codec = KafkaCodec::default();

        let request_handler = RequestHandler::default();

        tokio::spawn(async move {
            use futures::SinkExt;
            use tokio_stream::StreamExt;
            let mut framed = Framed::new(socket, codec);

            while let Some(req) = framed.next().await {
                match req {
                    Err(e) => warn!("Frame errored: {e}"),
                    Ok(req) => {
                        info!("Frame recieved {:?}", req);

                        let res = match request_handler.handle(req) {
                            Err(e) => {
                                error!("{:?}", e);
                                continue;
                            }
                            Ok(r) => r,
                        };

                        if let Err(e) = framed.send(res).await {
                            error!("Response Error: {:?}", e);
                        } else {
                            info!("Response Ok")
                        }
                    }
                }
            }

            info!("Socket disconnected {addr}");
        });
    }
}
