use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct ResponsePayload {
	sender: String,
	message: String
}

impl ResponsePayload {
	pub fn get_full_message(&self) -> String {
		format!("{}: {}", self.sender, self.message)
	}
}
