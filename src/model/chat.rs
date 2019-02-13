pub struct Chat {
	message: String
}

impl Chat {
	pub fn new(message: String) -> Self {
		Self {
			message,
		}
	}

	pub fn get_message(&self) -> String {
		self.message.clone()
	}
}
