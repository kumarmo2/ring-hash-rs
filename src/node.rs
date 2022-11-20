#[derive(Debug)]
pub struct Node {
    pub identifier: String,
}

impl Node {
    pub fn new(identifier: String) -> Self {
        Self { identifier }
    }

    pub fn from_str(identifier: &str) -> Self {
        Node {
            identifier: identifier.to_owned(),
        }
    }
}
