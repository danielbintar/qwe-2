use serde::{Serialize, Deserialize};

use super::super::place::Place;

#[derive(Serialize)]
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

#[derive(Deserialize)]
struct Character {
    id: usize
}

#[derive(Deserialize)]
pub struct ResponsePayload {
    character: Character,
    x: usize,
    y: usize,
    active_place: Place
}

impl ResponsePayload {
    pub fn get_id(&self) -> usize {
        self.character.id
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }

    pub fn get_active_place(&self) -> Place {
        self.active_place.clone()
    }
}
