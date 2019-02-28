use serde_derive::Deserialize;
use amethyst::ecs::{Component, DenseVecStorage};

pub struct Monster {
    id: usize
}

impl Monster {
    pub fn new(id: usize) -> Self {
        Self {
            id
        }
    }

    pub fn get_id(&self) -> usize {
        self.id.clone()
    }
}

impl Component for Monster {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Deserialize, Clone)]
pub struct Position {
    x: usize,
    y: usize
}

#[derive(Deserialize, Clone)]
pub struct MonsterPosition {
    id: usize,
    position: Position
}

impl MonsterPosition {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_x(&self) -> usize {
        self.position.x
    }

    pub fn get_y(&self) -> usize {
        self.position.y
    }
}
