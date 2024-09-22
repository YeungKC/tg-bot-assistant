use std::env;
use url::Url;

pub fn get_webhook_url_from_env() -> Option<Url> {
    env::var("ZEABUR_WEB_URL")
        .or_else(|_| env::var("WEBHOOK_URL"))
        .or_else(|_| env::var("RENDER_EXTERNAL_URL"))
        .ok()
        .and_then(|url| Url::parse(&url).ok())
}

pub fn get_secret_token_from_env() -> Option<String> {
    env::var("SECRET_TOKEN").ok()
}

pub fn get_port_from_env() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|port| port.parse().ok())
        .unwrap_or(8080)
}
