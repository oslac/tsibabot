use serde::Deserialize;

#[derive(Deserialize)]
pub struct Repository {
    full_name: String,
}

impl Repository {
    pub fn full_name(&self) -> &str {
        self.full_name.as_ref()
    }
}
