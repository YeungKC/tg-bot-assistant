use std::net::SocketAddr;

use teloxide::{
    adaptors::throttle::Limits,
    prelude::{LoggingErrorHandler, RequesterExt},
    update_listeners::webhooks,
    Bot,
};
use tracing::{error, info};
use url::Url;

use crate::{config, handlers, services, types::BotType};

pub fn create_bot() -> BotType {
    Bot::from_env().throttle(Limits::default()).cache_me()
}
pub trait BotExt {
    async fn run(&self) -> anyhow::Result<()>;
}

impl BotExt for BotType {
    async fn run(&self) -> anyhow::Result<()> {
        let bot = self.clone();
        tokio::spawn(async move {
            info!("Setting commands");
            if let Err(e) = handlers::set_commands(bot).await {
                error!("Failed to set commands: {}", e);
            }
        });

        let port = config::get_port_from_env();
        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        match config::get_webhook_url_from_env() {
            Some(url) => run_webhook_mode(self.clone(), url, addr).await,
            None => run_polling_mode(self.clone(), addr).await,
        }
    }
}

async fn run_polling_mode(bot: BotType, addr: SocketAddr) -> anyhow::Result<()> {
    info!("Running in polling mode");

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let server_future = axum::serve(listener, services::init()).with_graceful_shutdown(async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    });

    let mut dispatcher = handlers::create_dispatcher(bot);

    let bot_future = dispatcher.dispatch();

    tokio::select! {
        result = server_future => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
        _ = bot_future => {
            info!("Bot stopped");
        }
    }

    Ok(())
}

async fn run_webhook_mode(bot: BotType, url: Url, addr: SocketAddr) -> anyhow::Result<()> {
    info!("Running in webhook mode");

    let mut webhook_option = webhooks::Options::new(addr, url);
    if let Some(secret_token) = config::get_secret_token_from_env() {
        webhook_option = webhook_option.secret_token(secret_token);
    }

    let (update_listener, stop_flag, app) =
        webhooks::axum_to_router(bot.clone(), webhook_option).await?;

    let app = app.merge(services::init());

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let server_future = axum::serve(listener, app).with_graceful_shutdown(stop_flag);
    let mut dispatcher = handlers::create_dispatcher(bot);
    let bot_future = dispatcher.dispatch_with_listener(
        update_listener,
        LoggingErrorHandler::with_custom_text("An error from the update listener"),
    );

    tokio::select! {
        result = server_future => {
            if let Err(e) = result {
                error!("Server error: {}", e);
            }
        }
        _ = bot_future => {
            info!("Bot stopped");
        }
    }

    Ok(())
}
