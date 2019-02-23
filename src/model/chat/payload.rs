use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct RequestPayload {
    message: String
}

impl RequestPayload {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }
}

#[derive(Deserialize)]
struct ChatSender {
    name: String
}

#[derive(Deserialize)]
pub struct ResponsePayload {
    sender: ChatSender,
    message: String
}

impl ResponsePayload {
    pub fn get_full_message(&self) -> String {
        format!("{}: {}", self.sender.name, self.message)
    }
}
