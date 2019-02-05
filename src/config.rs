use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub url: String
}

impl Default for Request {
    fn default() -> Self {
        Request {
            url: "".to_string()
        }
    }
}
