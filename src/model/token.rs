use serde_derive::{Deserialize};

#[derive(Deserialize)]
pub struct Token {
	token: String
}

impl Token {
	pub fn get_token(&self) -> String {
		self.token.clone()
	}
}
