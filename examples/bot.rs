use log::info;
use tokio::signal;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::from_default_env()
                .add_directive("cookie_store=error".parse().unwrap())
                .add_directive("hyper_util=error".parse().unwrap()),
        )
        .init();

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
