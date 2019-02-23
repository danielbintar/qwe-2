use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPayload {
    direction: String
}

impl RequestPayload {
	pub fn new(x: f64, y: f64) -> Self {
		let direction =
			if x > 0.0 {
				"right"
			} else if x < 0.0 {
				"left"
			} else if y < 0.0 {
				"down"
			} else {
				"up"
			}.to_string();

		Self {
			direction
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsePayload {
    pub x: isize,
    pub y: isize
}
