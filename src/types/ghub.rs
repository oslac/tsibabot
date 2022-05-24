use self::ping_event::PingEvent;
use self::push_event::PushEvent;
use serde::Deserialize;
use std::fmt::Display;

pub mod commits;
pub mod ping_event;
pub mod push_event;
pub mod pusher;
pub mod repo;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum GitEvent {
    Push(PushEvent),
    Ping(PingEvent),
}

impl Display for GitEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            GitEvent::Ping(e) => write!(f, "{e}"),
            GitEvent::Push(e) => write!(f, "{e}"),
        }
    }
}
