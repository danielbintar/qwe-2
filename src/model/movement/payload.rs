use serde::Serialize;

#[derive(Serialize)]
pub struct RequestPayload {
    pub id: usize,
    pub x: isize,
    pub y: isize
}
