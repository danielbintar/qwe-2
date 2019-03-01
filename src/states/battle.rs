use amethyst::{
    prelude::*,
    ecs::Entity,
    core::Transform,
    renderer::{SpriteRender, Transparent, SpriteSheetHandle}
};

use crate::{
    general,
    config::Request,
    model::{
        token::Token,
        place::{Place, CurrentPlace},
        character::CharacterPosition,
        action::{Action, PlayerAction},
        monster::{Monster, MonsterPosition}
    }
};

use super::has_chat::HasChat;

use reqwest::header;

use serde_derive::Deserialize;

pub struct State {
    chat_button: Option<Entity>,
    chat_input: Option<Entity>,
    characters_position: Vec<CharacterPosition>,
    monsters_position: Vec<MonsterPosition>
}

impl State {
    pub fn new() -> Self {
        Self {
            chat_button: None,
            chat_input: None,
            characters_position: Vec::new(),
            monsters_position: Vec::new()
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

impl SimpleState for State {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.init_chat_ui(world);
        init_background(world);
        super::init_default_camera(world);
        world.add_resource(CurrentPlace{place: Some(Place::Battle)});
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        let world = data.world;
        self.handle_send_chat(world, event);
        Trans::None
    }
}

fn init_background(world: &mut World) {
    let sprite_sheet = super::load_sprite_sheet(world, "./resources/tiles/floor.png", "./resources/tiles/floor.ron");

    for i in 0..50 {
        for j in 0..50 {
            let mut transform = Transform::default();
            transform.set_x((i * general::GRID_SCALE_X) as f32);
            transform.set_y((j * general::GRID_SCALE_Y) as f32);
            transform.set_z(-10.0);

            let mut sprite_sheet = sprite_sheet.clone();

            let sprite = SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: 0,
            };
            world.create_entity().with(transform).with(sprite).build();
        }
    }
}
