use amethyst::{
    prelude::*,
    assets::{AssetStorage, Loader},
    core::{Parent, Transform},
    ecs::Entity,
    renderer::{
        Camera, PngFormat, Projection,
        SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
        Texture, TextureMetadata, Transparent
    }
};

use super::super::has_chat::HasChat;
use super::super::has_characters::HasCharacters;
use crate::components::player::Player;

pub struct State {
    chat_button: Option<Entity>,
    chat_input: Option<Entity>,
    chat_show: Option<Entity>
}

impl State {
    pub fn new() -> Self {
        Self {
            chat_button: None,
            chat_input: None,
            chat_show: None
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

    fn get_chat_show(&self) -> Entity {
        self.chat_show.unwrap()
    }

    fn set_chat_show(&mut self, e: Entity) {
        self.chat_show = Some(e)
    }
}

impl HasCharacters for State {

}

impl SimpleState for State {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.init_chat_ui(world);
        self.init_characters_ui(world);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        self.handle_receive_chat(data.world);
        Trans::None
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        self.handle_send_chat(data.world, event);
        Trans::None
    }
}
