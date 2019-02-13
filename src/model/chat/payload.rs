use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
	message: String
}

impl Payload {
	pub fn new(message: String) -> Self {
		Self {
			message,
		}
	}

	pub fn get_message(&self) -> String {
		self.message.clone()
	}

	pub fn get_full_message(&self) -> String {
		self.message.clone()
	}
}
