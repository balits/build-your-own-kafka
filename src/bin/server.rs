use kafka::broker::Broker;

use anyhow::Context;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("setting default subscriber failed")?;

    let broker = Broker::new().await;
    broker.run().await
}
