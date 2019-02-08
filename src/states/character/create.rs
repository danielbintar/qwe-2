use amethyst::prelude::*;
use amethyst::assets::{Loader};
use amethyst::ui::{UiTransform, Anchor, UiText, TextEditing, TtfFormat, UiButtonBuilder, UiEventType::Click};

use specs::Entity;

use crate::config::Request;
use crate::model::token::Token;
use crate::model::character::Character;
use crate::model::error::Response as ErrorResponse;
use crate::general;

use super::index::State as IndexState;

use std::vec::Vec;
use std::collections::HashMap;

use reqwest::header;

enum Buttons {
    Create,
    Back
}

#[derive(PartialEq, Eq, Hash)]
enum Texts {
    Name,
    Notice
}

pub struct State {
    ui_buttons: HashMap<Entity, Buttons>,
    ui_texts: HashMap<Texts, Entity>,
    create: bool,
}

impl State {
    pub fn new() -> Self {
        let btn_count = 2;
        let text_count = 2;

        Self {
            ui_buttons: HashMap::with_capacity(btn_count),
            ui_texts: HashMap::with_capacity(text_count),
            create: false
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

        let button_builder = UiButtonBuilder::new("create_button", "Create")
            .with_anchor(Anchor::TopMiddle)
            .with_position(0., -500.);
        let button = button_builder.build_from_world(world);
        self.ui_buttons.insert(button, Buttons::Create);


        let transform = UiTransform::new(
            "notice".to_string(), Anchor::TopMiddle,
            0., -450., 1., 400., 50., 0,
        );

        let notice = world
            .create_entity()
            .with(transform)
            .with(UiText::new(
                font.clone(),
                "".to_string(),
                [1., 1., 1., 1.],
                20.))
            .build();


        let transform = UiTransform::new(
            "name_label".to_string(), Anchor::TopMiddle,
            -250., -250., 1., 250., 50., 0
        );

        world
            .create_entity()
            .with(transform)
            .with(UiText::new(
                font.clone(),
                "NAME:".to_string(),
                [1., 1., 1., 1.],
                50.))
            .build();


        let transform = UiTransform::new(
            "name".to_string(), Anchor::TopMiddle,
            50., -250., 1., 400., 50., 0
        );

        let name = world
            .create_entity()
            .with(transform)
            .with(UiText::new(
                font.clone(),
                "".to_string(),
                [1., 1., 1., 1.],
                50.))
            .with(TextEditing::new(
                10,
                [1., 1., 1., 1.],
                [0.0, 0.0, 0.0, 1.0],
                false))
            .build();

        self.ui_texts.insert(Texts::Notice, notice);
        self.ui_texts.insert(Texts::Name, name);

        let button_builder = UiButtonBuilder::new("back_button", "BACK")
            .with_anchor(Anchor::TopMiddle)
            .with_position(0., -600.);
        let button = button_builder.build_from_world(world);
        self.ui_buttons.insert(button, Buttons::Back);
    }

    fn back(&mut self, world: &mut World) -> SimpleTrans {
        world.delete_all();
        Trans::Pop
    }


    fn prepare_create(&self, world: &mut World) -> HashMap<String, String> {
        let mut ui_text_storage = world.write_storage::<UiText>();
        ui_text_storage.get_mut(*self.ui_texts.get(&Texts::Notice).unwrap()).unwrap().text = "Request create to server".to_string();

        let mut map = HashMap::new();
        let name = ui_text_storage.get(*self.ui_texts.get(&Texts::Name).unwrap()).unwrap().text.clone();
        map.insert("name".to_string(), name);
        map
    }

    fn after_create(&mut self, world: &mut World, notice: String) {
        let mut ui_text_storage = world.write_storage::<UiText>();
        ui_text_storage.get_mut(*self.ui_texts.get(&Texts::Notice).unwrap()).unwrap().text = notice.to_string();
    }

    fn perform_create(&self, form: HashMap<String, String>, world: &mut World) -> std::result::Result<reqwest::Response, reqwest::Error> {
        let config = world.read_resource::<Request>();
        let uri = format!("{}{}", config.url, "/my-characters");

        let mut headers = header::HeaderMap::new();
        let token = format!("Bearer {}", world.read_resource::<Token>().get_token());
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&token).unwrap());

        reqwest::Client::builder()
            .default_headers(headers)
            .build()?
            .post(&uri)
            .json(&form)
            .send()
    }

    fn create(&mut self, world: &mut World) {
        let form = self.prepare_create(world);
        let resp = self.perform_create(form, world);
        let notice = match resp {
            Ok(mut resp) => {
                if resp.status().is_success() {
                    self.create = true;
                    "Success".to_string()
                } else if resp.status().is_server_error() {
                    "Server is maintenance".to_string()
                } else {
                    let err: ErrorResponse = resp.json().unwrap();
                    err.get_error()
                }
            },
            Err(_) => "Server is maintenance".to_string()
        };
        self.after_create(world, notice)
    }
}

impl SimpleState for State {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.initialize_ui(world);
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if self.create {
            data.world.delete_all();
            return Trans::Switch(Box::new({
                IndexState::new()
            }))
        }

        match event {
            StateEvent::Ui(x) => match x.event_type {
                Click => {
                    if let Some(button) = self.ui_buttons.get(&x.target) {
                        match button {
                            Buttons::Back => return self.back(data.world),
                            Buttons::Create => self.create(data.world)
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
