use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPayload {
    pub x: isize,
    pub y: isize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponsePayload {
    pub x: isize,
    pub y: isize
}
