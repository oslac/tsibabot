use crate::config::Config;
use crate::http::TBot;
use crate::types::ghub::GitEvent;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Extension, Router};
use teloxide::prelude::Requester;

pub fn router() -> Router {
    Router::new().route("/hook", post(handler))
}

async fn handler(
    ctx: Extension<Config>,
    bot: Extension<TBot>,
    event: GitEvent,
) -> impl IntoResponse {
    send(&ctx, &bot, event).await
}

async fn send(ctx: &Config, bot: &TBot, event: GitEvent) -> StatusCode {
    match bot.send_message(ctx.telegram.chat_id().to_string(), event.to_string()).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
