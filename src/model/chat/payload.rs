use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
	sender: String,
	message: String
}

impl Payload {
	pub fn new(message: String) -> Self {
		Self {
			sender: "".to_string(),
			message,
		}
	}

	pub fn get_message(&self) -> String {
		self.message.clone()
	}

	pub fn get_full_message(&self) -> String {
		format!("{}: {}", self.sender, self.message)
	}
}
