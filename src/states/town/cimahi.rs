use amethyst::{
    prelude::*,
    ecs::Entity,
};

use super::{
    IsTown,
    super::{
        has_chat::HasChat,
        has_characters::HasCharacters
    }
};

use crate::model::{
    character::CharacterPosition,
    action::{Action, PlayerAction}
};

pub struct State {
    chat_button: Option<Entity>,
    chat_input: Option<Entity>,
    characters_position: Vec<CharacterPosition>
}

impl State {
    pub fn new() -> Self {
        Self {
            chat_button: None,
            chat_input: None,
            characters_position: Vec::new()
        }
    }
}

impl HasChat for State {
    fn get_chat_button(&self) -> Entity {
        self.chat_button.unwrap()
    }

    fn set_chat_button(&mut self, e: Entity) {
        self.chat_button = Some(e)
    }

    fn get_chat_input(&self) -> Entity {
        self.chat_input.unwrap()
    }

    fn set_chat_input(&mut self, e: Entity) {
        self.chat_input = Some(e)
    }
}

impl HasCharacters for State {
    fn set_characters_position(&mut self, c: Vec<CharacterPosition>) {
        self.characters_position = c;
    }

    fn get_characters_position(&self) -> Vec<CharacterPosition> {
        self.characters_position.clone()
    }
}

impl IsTown for State {
    fn get_town_id(&self) -> usize {
        1
    }
}

impl SimpleState for State {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.init_town(world);
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        self.handle_send_chat(data.world, event);
        Trans::None
    }

    fn fixed_update(&mut self, data: StateData<GameData>) -> SimpleTrans {
        let world = data.world;
        if is_leaving(world) {
            world.delete_all();
        }

        Trans::None
    }
}

fn is_leaving(world: &mut World) -> bool {
    let mut action = world.write_resource::<Action>();
    if let Some(PlayerAction::LeaveTown) = action.action.take() {
        return true;
    }
    false
}
