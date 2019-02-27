pub mod cimahi;

use amethyst::{
    prelude::*,
    core::Transform,
    renderer::{SpriteRender}
};

use crate::{
    general,
    config::Request,
    model::{
        token::Token,
        character::CharacterPosition,
        place::{Place, CurrentPlace}
    }
};

use reqwest::header;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Town {
    characters: Vec<CharacterPosition>
}

trait IsTown : super::has_characters::HasCharacters + super::has_chat::HasChat {
    fn get_town_id(&self) -> usize;

    fn init_town(&mut self, world: &mut World) {
        world.add_resource(crate::systems::outgoing_movement::AllowMoving{allowed: true});
        let resp = self.request_town(world);
        match resp {
            Ok(mut resp) => {
                if resp.status().is_success() {
                    let town: Town = resp.json().unwrap();
                    self.set_characters_position(town.characters);
                } else if resp.status().is_server_error() {
                    panic!()
                } else {
                    panic!()
                }
            },
            Err(_) => panic!()
        };

        self.init_chat_ui(world);
        self.init_background(world);
        self.init_characters_ui(world);
        world.add_resource(CurrentPlace{place: Some(Place::Town)});
    }

    fn init_background(&self, world: &mut World) {
        let portal_sprite_sheet = super::load_sprite_sheet(world, "./resources/tiles/portal.png", "./resources/tiles/portal.ron");
        let sprite_sheet = super::load_sprite_sheet(world, "./resources/tiles/floor.png", "./resources/tiles/floor.ron");

        for i in 0..50 {
            for j in 0..50 {
                let mut transform = Transform::default();
                transform.set_x((i * general::GRID_SCALE_X) as f32);
                transform.set_y((j * general::GRID_SCALE_Y) as f32);
                transform.set_z(-10.0);

                let mut sprite_sheet = sprite_sheet.clone();
                if j > 10 && j < 15 && ((i < 7) || (i > 42 && i <= 50)) {
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

    fn request_town(&self, world: &mut World) -> std::result::Result<reqwest::Response, reqwest::Error> {
        let config = world.read_resource::<Request>();
        let uri = format!("{}{}{}", config.api_url, "/towns/", self.get_town_id());

        let mut headers = header::HeaderMap::new();
        let token = format!("Bearer {}", world.read_resource::<Token>().get_token());
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&token).unwrap());

        reqwest::Client::builder()
            .default_headers(headers)
            .build()?
            .get(&uri)
            .send()
    }
}
