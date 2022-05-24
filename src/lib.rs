#![warn(unreachable_pub, private_in_public)]
pub mod config;
pub mod http;
pub mod types;

use teloxide::prelude::RequesterExt;
use teloxide::types::ParseMode;
use teloxide::Bot;

pub async fn try_main() -> anyhow::Result<()> {
    env_logger::init();
    let config = config::load();
    let bot = Bot::from_env().parse_mode(ParseMode::MarkdownV2).auto_send();

    http::serve(config, bot).await
}
