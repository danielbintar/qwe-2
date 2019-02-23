use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Show ;

impl Component for Show {
    type Storage = NullStorage<Self>;
}
