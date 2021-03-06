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

use super::{
    super::{
        has_chat::HasChat,
        has_characters::HasCharacters,
        town::cimahi::State as CimahiState,
    }
};

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

    fn init_monsters_ui(&self, world: &mut World) {
        let monsters = self.monsters_position.clone();

        for monster in &monsters {
            let sprite = super::super::load_sprite_sheet(world, "./resources/sprites/monster.png", "./resources/sprites/monster.ron");
            let _monster = init_monster(world, &sprite, monster);
        }
    }
}

fn init_monster(world: &mut World, sprite_sheet: &SpriteSheetHandle, monster: &MonsterPosition) -> Entity {
    let mut transform = Transform::default();
    transform.set_x((monster.get_x() * general::GRID_SCALE_X) as f32);
    transform.set_y((monster.get_y() * general::GRID_SCALE_Y) as f32);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Monster::new(monster.get_id()))
        .with(Transparent)
        .build()
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

#[derive(Deserialize)]
pub struct Region {
    characters: Vec<CharacterPosition>,
    monsters: Vec<MonsterPosition>
}

impl SimpleState for State {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.init_chat_ui(world);
        init_background(world);
        let resp = request_region(world);
        match resp {
            Ok(mut resp) => {
                if resp.status().is_success() {
                    let region: Region = resp.json().unwrap();
                    self.set_characters_position(region.characters);
                    self.monsters_position = region.monsters;
                } else if resp.status().is_server_error() {
                    panic!()
                } else {
                    panic!()
                }
            },
            Err(_) => panic!()
        };
        self.init_characters_ui(world);
        self.init_monsters_ui(world);
        world.add_resource(crate::systems::outgoing_movement::AllowMoving{allowed: true});
        world.add_resource(CurrentPlace{place: Some(Place::Region)});
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        let world = data.world;
        self.handle_send_chat(world, event);
        Trans::None
    }

    fn fixed_update(&mut self, data: StateData<GameData>) -> SimpleTrans {
        let world = data.world;
        if is_leaving(world) {
            world.add_resource(crate::systems::outgoing_movement::AllowMoving{allowed: false});
            world.delete_all();
            return Trans::Switch(Box::new({
                CimahiState::new()
            }))
        }

        Trans::None
    }
}

fn request_region(world: &mut World) -> std::result::Result<reqwest::Response, reqwest::Error> {
    let config = world.read_resource::<Request>();
    let uri = format!("{}{}", config.api_url, "/regions/1");

    let mut headers = header::HeaderMap::new();
    let token = format!("Bearer {}", world.read_resource::<Token>().get_token());
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&token).unwrap());

    reqwest::Client::builder()
        .default_headers(headers)
        .build()?
        .get(&uri)
        .send()
}

fn init_background(world: &mut World) {
    let portal_sprite_sheet = super::super::load_sprite_sheet(world, "./resources/tiles/portal.png", "./resources/tiles/portal.ron");
    let sprite_sheet = super::super::load_sprite_sheet(world, "./resources/tiles/floor.png", "./resources/tiles/floor.ron");

    for i in 0..50 {
        for j in 0..50 {
            let mut transform = Transform::default();
            transform.set_x((i * general::GRID_SCALE_X) as f32);
            transform.set_y((j * general::GRID_SCALE_Y) as f32);
            transform.set_z(-10.0);

            let mut sprite_sheet = sprite_sheet.clone();
            if j >= 15 && j <= 18 && i >= 15 && i <= 18 {
                sprite_sheet = portal_sprite_sheet.clone();
            }

            let sprite = SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: 0,
            };
            world.create_entity().with(transform).with(sprite).build();
        }
    }
}

fn is_leaving(world: &mut World) -> bool {
    let mut action = world.write_resource::<Action>();
    if let Some(PlayerAction::LeaveRegion) = action.action.take() {
        return true;
    }
    false
}
