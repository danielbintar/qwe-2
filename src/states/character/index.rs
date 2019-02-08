use amethyst::prelude::*;
use amethyst::assets::{Loader};
use amethyst::ui::{UiTransform, Anchor, UiText, TtfFormat};

use crate::config::Request;
use crate::model::token::Token;
use crate::model::character::Character;

use std::vec::Vec;

use reqwest::header;

pub struct State {
    characters: Vec<Character>
}

impl State {
    pub fn new() -> Self {
        Self {
            characters: Vec::new()
        }
    }

    fn initialize_ui(&mut self, world: &mut World) {
        let font = world.read_resource::<Loader>().load(
            "./resources/font/square.ttf",
            TtfFormat,
            Default::default(),
            (),
            &world.read_resource(),
        );

        let transform = UiTransform::new(
            "title".to_string(), Anchor::TopMiddle,
            0., -100., 1., 250., 50., 0,
        );

        world
            .create_entity()
            .with(transform)
            .with(UiText::new(
                font.clone(),
                "Character".to_string(),
                [1., 1., 1., 1.],
                50.))
            .build();
    }

    fn request_characters(&self, world: &mut World) -> std::result::Result<reqwest::Response, reqwest::Error> {
        let config = world.read_resource::<Request>();
        let uri = format!("{}{}", config.url, "/my-characters");

        let mut headers = header::HeaderMap::new();
        let token = format!("Bearer {}", world.read_resource::<Token>().get_token());
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&token).unwrap());

        reqwest::Client::builder()
            .default_headers(headers)
            .build()?
            .get(&uri)
            .send()
    }

    fn initialize_characters(&mut self, world: &mut World) {
        let resp = self.request_characters(world);
        match resp {
            Ok(mut resp) => {
                if resp.status().is_success() {
                    self.characters = resp.json().unwrap();
                } else if resp.status().is_server_error() {
                    panic!()
                } else {
                    panic!()
                }
            },
            Err(_) => panic!()
        };
    }
}

impl SimpleState for State {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.initialize_ui(world);
        self.initialize_characters(world);
    }
}
