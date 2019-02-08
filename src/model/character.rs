use serde_derive::{Deserialize};

#[derive(Deserialize)]
pub struct Character {
	id: usize,
	name: String
}

impl Character {
	pub fn get_id(&self) -> usize {
		self.id.clone()
	}

	pub fn get_name(&self) -> String {
		self.name.clone()
	}
}
