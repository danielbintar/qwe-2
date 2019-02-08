use serde_derive::{Deserialize};

#[derive(Deserialize)]
pub struct Response {
	error: String
}

impl Response {
	pub fn get_error(&self) -> String {
		self.error.clone()
	}
}
