#[tokio::main]
async fn main() -> Result<(), openwechat::Error> {
    tracing_subscriber::fmt::init();

    openwechat::bootstrap::run().await
}
