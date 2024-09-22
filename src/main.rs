mod bot;
mod config;
mod handlers;
mod services;
mod types;

use bot::BotExt;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting application");

    bot::create_bot().run().await
}
