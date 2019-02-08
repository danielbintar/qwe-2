use amethyst::prelude::*;
use amethyst::assets::{Loader};
use amethyst::ui::{UiTransform, Anchor, UiText, TtfFormat, UiButtonBuilder};

use specs::Entity;

use crate::config::Request;
use crate::model::token::Token;
use crate::model::character::Character;
use crate::general;

use std::vec::Vec;
use std::collections::HashMap;

use reqwest::header;

enum Buttons {
    Create,
    Logout
}

pub struct State {
    characters: Vec<Character>,
    ui_buttons: HashMap<Entity, Buttons>
}

impl State {
    pub fn new() -> Self {
        let btn_count = 1;

        Self {
            characters: Vec::new(),
            ui_buttons: HashMap::with_capacity(btn_count)
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


        for i in 0..general::MAX_MY_CHARACTER {
            println!("{}", i);
            let x: i32 = (i as i32 - 2) * 200;

            if self.characters.len() <= i {
                let button_builder = UiButtonBuilder::new("create_button", "Create")
                    .with_anchor(Anchor::TopMiddle)
                    .with_position(x as f32, -400.);
                let button = button_builder.build_from_world(world);
                self.ui_buttons.insert(button, Buttons::Create);
            } else {
                let transform = UiTransform::new(
                    self.characters[i as usize].get_name(), Anchor::TopMiddle,
                    x as f32, -300., 1., 250., 50., 0,
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

                let button_builder = UiButtonBuilder::new("play_button", "Enter")
                    .with_anchor(Anchor::TopMiddle)
                    .with_position(x as f32, -400.);
                let button = button_builder.build_from_world(world);
                self.ui_buttons.insert(button, Buttons::Create);
            }
        }

        let button_builder = UiButtonBuilder::new("logout_button", "LOGOUT")
            .with_anchor(Anchor::TopMiddle)
            .with_position(0., -600.);
        let button = button_builder.build_from_world(world);
        self.ui_buttons.insert(button, Buttons::Logout);
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

        self.initialize_characters(world);
        self.initialize_ui(world);
    }
}
