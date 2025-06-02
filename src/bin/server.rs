use kafka::broker::Broker;

use anyhow::{bail, Context};
use tracing_subscriber::FmtSubscriber;
use tracing::error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        //.with_max_level(tracing::Level::ERROR)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .context("setting default subscriber failed")?;

    let broker = Broker::new().await?;
    if let Err(e) = broker.run().await {
        error!("Broker's event loop returned an error: {}", e);
        bail!("Broker's event loop returned an error: {}", e)
    } else {
        Ok(())
    }
}
