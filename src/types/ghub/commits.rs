use serde::Deserialize;

#[derive(Deserialize)]
pub struct HeadCommit {
    message: String,
}

impl HeadCommit {
    pub fn message(&self) -> &str {
        self.message.as_ref()
    }
}
