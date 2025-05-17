use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:9092").await?;

    match listener.accept().await {
        Ok((_stream, _addr)) => println!("accept new connection"),
        Err(e) => println!("error: {}", e),
    }

    Ok(())
}
