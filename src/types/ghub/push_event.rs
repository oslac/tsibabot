use super::commits::HeadCommit;
use super::pusher::Pusher;
use super::repo::Repository;
use serde::Deserialize;
use std::fmt::Display;
use teloxide::utils::markdown::escape;

#[derive(Deserialize)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    reference: String,
    repository: Repository,
    pusher: Pusher,
    head_commit: HeadCommit,
}

impl PushEvent {
    pub fn reference(&self) -> &str {
        self.reference.as_ref()
    }
}

impl Display for PushEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = escape(self.repository.full_name());
        let p = escape(self.pusher.name());
        let m = escape(self.head_commit.message());
        let push_event = format!("New push 🚨:\n  *repo*: `{r}`\n  *by*: {p}\n  *msg*:'{m}'");
        write!(f, "{push_event}")
    }
}
