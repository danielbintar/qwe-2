use amethyst::{
    prelude::*,
    assets::Loader,
    ui::{UiTransform, Anchor, UiText, TtfFormat, UiButtonBuilder, UiEventType::Click},
    ecs::Entity
};

use crate::config::Request;
use crate::model::token::Token;
use crate::model::character::Character;
use crate::general;

use std::vec::Vec;
use std::collections::HashMap;
use std::thread;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

use reqwest::header;

use ws::connect;

use super::super::auth::login::State as LoginState;
use super::super::town::cimahi::State as CimahiState;
use super::create::State as CreateState;

enum Buttons {
    Create,
    Enter(Character),
    Logout
}

pub struct State {
    characters: Vec<Character>,
    ui_buttons: HashMap<Entity, Buttons>
}

impl State {
    pub fn new() -> Self {
        let btn_count = 1 + general::MAX_MY_CHARACTER;

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
                        self.characters[i as usize].get_name(),
                        [1., 1., 1., 1.],
                        50.))
                    .build();

                let button_builder = UiButtonBuilder::new("play_button", "Enter")
                    .with_anchor(Anchor::TopMiddle)
                    .with_position(x as f32, -400.);
                let button = button_builder.build_from_world(world);
                self.ui_buttons.insert(button, Buttons::Enter(self.characters[i].clone()));
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
        let uri = format!("{}{}", config.api_url, "/my-characters");

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

    fn logout(&self, world: &mut World) -> SimpleTrans {
        world.delete_all();
        Trans::Switch(Box::new({
            LoginState::new()
        }))
    }

    fn create(&self, world: &mut World) -> SimpleTrans {
        world.delete_all();
        Trans::Push(Box::new({
            CreateState::new()
        }))
    }

    fn enter(&self, world: &mut World, character: Character) -> SimpleTrans {
        request_enter(world, character.get_id());

        let (tx_receive, rx_receive) = mpsc::channel();
        let (tx_send, rx_send) = mpsc::channel();
        let sender = Arc::new(Mutex::new(tx_send));
        let receiver = Arc::new(Mutex::new(rx_receive));
        let r = crate::model::chat::resource::Resource::new(Arc::clone(&sender), Arc::clone(&receiver));
        let token = format!("Bearer {}", world.read_resource::<Token>().get_token());
        world.add_resource(r);
        world.add_resource(character);

        let uri = get_chat_link(world);
        thread::spawn(move || {
            connect(uri, |out| crate::model::chat::client::Client::new(out, &tx_receive, &rx_send, token.clone()) ).unwrap()
        });

        world.delete_all();
        Trans::Switch(Box::new({
            CimahiState::new()
        }))
    }
}

fn request_enter(world: &mut World, id: usize) {
    let config = world.read_resource::<Request>();
    let uri = format!("{}{}{}/play", config.api_url, "/my-characters/", id);

    let mut headers = header::HeaderMap::new();
    let token = format!("Bearer {}", world.read_resource::<Token>().get_token());
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&token).unwrap());

    reqwest::Client::builder()
        .default_headers(headers)
        .build().unwrap()
        .post(&uri)
        .send()
        .unwrap();
}

fn get_chat_link(world: &mut World) -> String {
    let config = world.read_resource::<Request>();
    format!("{}{}", config.ws_url, "/chat")
}

impl SimpleState for State {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.initialize_characters(world);
        self.initialize_ui(world);
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.initialize_characters(world);
        self.initialize_ui(world);
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Ui(x) => match x.event_type {
                Click => {
                    if let Some(button) = self.ui_buttons.get(&x.target) {
                        match button {
                            Buttons::Logout => return self.logout(data.world),
                            Buttons::Create => return self.create(data.world),
                            Buttons::Enter(y) => return self.enter(data.world, (*y).clone())
                        }
                    }
                },
                _ => (),
            },
            _ => (),
        }
        Trans::None
    }
}
