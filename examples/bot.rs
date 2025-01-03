use log::info;
use tokio::signal;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Err(err) = openwechat::bootstrap::run().await {
        panic!("Failed to run bot: {}", err);
    }

    match signal::ctrl_c().await {
        Ok(_) => {
            info!("Ctrl-C received, shutting down");
        }
        Err(e) => {
            panic!("Failed to listen for Ctrl-C: {}", e);
        }
    }
}
