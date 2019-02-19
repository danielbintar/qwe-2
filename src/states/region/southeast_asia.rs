use amethyst::prelude::*;

use super::IsRegion;

pub struct State;

impl SimpleState for State {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        self.init_region(world);
    }
}

impl IsRegion for State {
    fn get_region_id(&self) -> usize {
        1
    }
}
