use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct RequestPayload {
	pub id: usize,
    pub x: isize,
    pub y: isize
}
