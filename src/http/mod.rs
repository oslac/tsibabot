use crate::config::Config;

use anyhow::Context;
use axum::{Extension, Router, Server};
use teloxide::adaptors::{AutoSend, DefaultParseMode};
use teloxide::Bot;
use tower_http::trace::TraceLayer;

pub mod extractor;
pub mod routes;

pub type TBot = AutoSend<DefaultParseMode<Bot>>;

pub async fn serve(config: Config, bot: TBot) -> anyhow::Result<()> {
    let addr = config.server.addr;
    let app = routes()
        .layer(TraceLayer::new_for_http())
        .layer(Extension(config))
        .layer(Extension(bot))
        .into_make_service();

    Server::bind(&addr).serve(app).await.context("ERROR: while running the HTTP Server")
}

fn routes() -> Router {
    Router::new().merge(routes::HealthAPI()).merge(routes::HookAPI())
}
