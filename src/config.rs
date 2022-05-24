use std::env;
use std::net::SocketAddr;

pub fn load() -> Config {
    dotenv::dotenv().ok();

    let port = env::var("PORT").ok().map_or(Ok(8080), |val| val.parse::<u16>()).unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let server = ServerConfig { port, addr };

    let token = env::var("TELOXIDE_TOKEN").unwrap();
    let chat_id = env::var("CHAT_ID").unwrap();
    let telegram = TelegramConfig { token, chat_id };

    let hmac_key = env::var("HMAC").unwrap();
    let github = GithubConfig { hmac_key };

    Config { server, telegram, github }
}

#[derive(Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub telegram: TelegramConfig,
    pub github: GithubConfig,
}

#[derive(Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub addr: SocketAddr,
}

#[derive(Clone)]
pub struct TelegramConfig {
    token: String,
    chat_id: String,
}

#[derive(Clone)]
pub struct GithubConfig {
    /// HMAC verification key
    hmac_key: String,
}

impl GithubConfig {
    pub fn hmac_key(&self) -> &str {
        &self.hmac_key
    }
}

impl TelegramConfig {
    pub fn chat_id(&self) -> &str {
        &self.chat_id
    }
    pub fn token(&self) -> &str {
        &self.token
    }
}
