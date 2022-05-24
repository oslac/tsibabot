use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pusher {
    name: String,
}

impl Pusher {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
