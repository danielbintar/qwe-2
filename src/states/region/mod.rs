pub mod southeast_asia;

use amethyst::{
    prelude::*,
    core::Transform,
    renderer::{SpriteRender}
};

use reqwest::header;

use serde_derive::{Deserialize};

use crate::config::Request;

use crate::general;
use crate::model::token::Token;
use crate::model::character::CharacterPosition;

#[derive(Deserialize)]
pub struct Region {
    characters: Vec<CharacterPosition>
}

trait IsRegion : super::has_characters::HasCharacters + super::has_chat::HasChat {
    fn get_region_id(&self) -> usize;

    fn init_region(&mut self, world: &mut World) {
        world.add_resource(crate::systems::movement::AllowMoving{allowed: true});
        let resp = self.request_region(world);
        match resp {
            Ok(mut resp) => {
                if resp.status().is_success() {
                    let region: Region = resp.json().unwrap();
                    self.set_characters_position(region.characters);
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
    }

    fn init_background(&self, world: &mut World) {
        let floor_sprite_sheet = super::load_sprite_sheet(world, "./resources/tiles/floor.png", "./resources/tiles/floor.ron");
        let portal_sprite_sheet = super::load_sprite_sheet(world, "./resources/tiles/portal.png", "./resources/tiles/portal.ron");

        for i in 0..50 {
            for j in 0..50 {
                let mut transform = Transform::default();
                transform.set_x((j * general::GRID_SCALE_X) as f32);
                transform.set_y((i * general::GRID_SCALE_Y) as f32);
                transform.set_z(-10.0);
                let mut sprite_sheet = floor_sprite_sheet.clone();
                if i > 10 && i < 15 && ((j < 7) || (j > 42 && j <= 50)) {
                    sprite_sheet = portal_sprite_sheet.clone();
                }
                let sprite = SpriteRender {
                    sprite_sheet: sprite_sheet,
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