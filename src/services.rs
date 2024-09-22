use axum::routing::get;

pub fn init() -> axum::Router {
    axum::Router::new().route("/", get(pong))
}

async fn pong() -> &'static str {
    "Pong"
}
