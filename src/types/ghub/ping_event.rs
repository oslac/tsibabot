use super::repo::Repository;

use serde::Deserialize;
use std::fmt::Display;
use teloxide::utils::markdown::escape;

#[derive(Deserialize)]
pub struct PingEvent {
    zen: String,
    hook_id: u64,
    repository: Repository,
}

impl PingEvent {
    pub fn zen(&self) -> &str {
        self.zen.as_ref()
    }
    pub fn hook_id(&self) -> u64 {
        self.hook_id
    }
    pub fn repository(&self) -> &str {
        self.repository.full_name()
    }
}

impl Display for PingEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let zen = escape(self.zen());
        let repo = escape(self.repository());
        let ping_event = format!("🚨 New Ping:\n  *zen*: _{}_\n  *repo*: `{}`", zen, repo);
        write!(f, "{ping_event}")
    }
}
