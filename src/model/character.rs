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

#[derive(Deserialize, Clone)]
pub struct CharacterPosition {
    id: usize,
    x: usize,
    y: usize
}

impl CharacterPosition {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }
}
