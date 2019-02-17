use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Player {
    id: usize
}

impl Player {
    pub fn new(id: usize) -> Self {
        Self {
            id
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
