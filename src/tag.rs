use serde::Serialize;

#[derive(Serialize)]
pub struct Tag(String);

impl Tag {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}
