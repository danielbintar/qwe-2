use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub api_url: String,
    pub ws_url: String
}

impl Default for Request {
    fn default() -> Self {
        Request {
            api_url: "".to_string(),
            ws_url: "".to_string()
        }
    }
}
