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
}
