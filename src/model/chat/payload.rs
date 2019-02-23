use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
struct ChatSender {
    id: usize,
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsePayload {
    sender: ChatSender,
    message: String
}

impl ResponsePayload {
    pub fn get_full_message(&self) -> String {
        format!("{}: {}", self.sender.name, self.message)
    }
}
