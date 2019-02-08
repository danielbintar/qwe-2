use amethyst::prelude::*;
use amethyst::assets::{Loader};
use amethyst::ui::{UiTransform, Anchor, UiText, TtfFormat, TextEditing, UiButtonBuilder, UiEventType::Click};

use specs::Entity;

use std::collections::HashMap;

use crate::config::Request;
use crate::model::token::Token;
use super::register::State as RegisterState;

enum Buttons {
    Login,
    Register
}

#[derive(PartialEq, Eq, Hash)]
enum Texts {
    Username,
    Password,
    Notice
}

pub struct State {
    ui_buttons: HashMap<Entity, Buttons>,
    ui_texts: HashMap<Texts, Entity>
}

impl State {
    pub fn new() -> Self {
        let btn_count = 2;
        let text_count = 3;

        Self {
            ui_buttons: HashMap::with_capacity(btn_count),
            ui_texts: HashMap::with_capacity(text_count)
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
            0., -100., 1., 200., 50., 0,
        );

        world
            .create_entity()
            .with(transform)
            .with(UiText::new(
                font.clone(),
                "Immortal".to_string(),
                [1., 1., 1., 1.],
                50.))
            .build();


        let button_builder = UiButtonBuilder::new("login_button", "Login")
            .with_anchor(Anchor::TopMiddle)
            .with_position(0., -500.);
        let button = button_builder.build_from_world(world);
        self.ui_buttons.insert(button, Buttons::Login);

        let button_builder = UiButtonBuilder::new("register_button", "Register")
            .with_anchor(Anchor::TopMiddle)
            .with_position(0., -620.)
            .with_size(200., 50.);
        let button = button_builder.build_from_world(world);
        self.ui_buttons.insert(button, Buttons::Register);


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
            "username_label".to_string(), Anchor::TopMiddle,
            -250., -250., 1., 250., 50., 0
        );

        world
            .create_entity()
            .with(transform)
            .with(UiText::new(
                font.clone(),
                "USERNAME:".to_string(),
                [1., 1., 1., 1.],
                50.))
            .build();


        let transform = UiTransform::new(
            "username".to_string(), Anchor::TopMiddle,
            50., -250., 1., 400., 50., 0
        );

        let username = world
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


        let transform = UiTransform::new(
            "password_label".to_string(), Anchor::TopMiddle,
            -250., -350., 1., 250., 50., 0
        );

        world
            .create_entity()
            .with(transform)
            .with(UiText::new(
                font.clone(),
                "PASSWORD:".to_string(),
                [1., 1., 1., 1.],
                50.))
            .build();


        let transform = UiTransform::new(
            "password".to_string(), Anchor::TopMiddle,
            50., -350., 1., 400., 50., 0
        );

        let password = world
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
        self.ui_texts.insert(Texts::Username, username);
        self.ui_texts.insert(Texts::Password, password);

        let mut ui_text_storage = world.write_storage::<UiText>();
        ui_text_storage.get_mut(password).unwrap().password = true;
    }

    fn prepare_login(&self, world: &mut World) -> HashMap<String, String> {
        let mut ui_text_storage = world.write_storage::<UiText>();
        ui_text_storage.get_mut(*self.ui_texts.get(&Texts::Notice).unwrap()).unwrap().text = "Request login to server".to_string();

        let mut map = HashMap::new();
        let username = ui_text_storage.get(*self.ui_texts.get(&Texts::Username).unwrap()).unwrap().text.clone();
        let password = ui_text_storage.get(*self.ui_texts.get(&Texts::Password).unwrap()).unwrap().text.clone();
        map.insert("username".to_string(), username);
        map.insert("password".to_string(), password);
        map
    }

    fn after_login(&self, world: &mut World, notice: String) {
        let mut ui_text_storage = world.write_storage::<UiText>();
        ui_text_storage.get_mut(*self.ui_texts.get(&Texts::Notice).unwrap()).unwrap().text = notice.to_string();
    }

    fn perform_login(&self, form: HashMap<String, String>, world: &mut World) -> std::result::Result<reqwest::Response, reqwest::Error> {
        let config = world.read_resource::<Request>();
        let uri = format!("{}{}", config.url, "/users/sign_in");

        reqwest::Client::new()
            .post(&uri)
            .json(&form)
            .send()
    }

    fn login(&self, world: &mut World) {
        let form = self.prepare_login(world);
        let resp = self.perform_login(form, world);
        let notice = match resp {
            Ok(mut resp) => {
                if resp.status().is_success() {
                    let token: Token = resp.json().unwrap();
                    world.add_resource(token);
                    "Login Success"
                } else if resp.status().is_server_error() {
                    "Server is maintenance"
                } else {
                    "Wrong username or password"
                }
            },
            Err(_) => "Server is maintenance"
        };
        self.after_login(world, notice.to_string())
    }

}

impl SimpleState for State {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.initialize_ui(world);
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Ui(x) => match x.event_type {
                Click => {
                    if let Some(button) = self.ui_buttons.get(&x.target) {
                        match button {
                            Buttons::Login => self.login(data.world),
                            Buttons::Register => return register(data.world)
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

fn register(world: &mut World) -> SimpleTrans {
    world.delete_all();
    Trans::Switch(Box::new({
        RegisterState::new()
    }))
}
