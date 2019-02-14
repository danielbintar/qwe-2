use amethyst::{
    prelude::*,
    assets::Loader,
    ui::{UiTransform, Anchor, UiText, TtfFormat, UiButtonBuilder, TextEditing, UiEventType::Click},
    ecs::Entity
};

use std::collections::HashMap;

use crate::config::Request;
use super::login::State as LoginState;

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
                "Register".to_string(),
                [1., 1., 1., 1.],
                50.))
            .build();


        let button_builder = UiButtonBuilder::new("register_button", "Register")
                .with_anchor(Anchor::TopMiddle)
                .with_position(0., -500.);
            let button = button_builder.build_from_world(world);
            self.ui_buttons.insert(button, Buttons::Register);

        let button_builder = UiButtonBuilder::new("login_button", "Login")
            .with_anchor(Anchor::TopMiddle)
            .with_position(0., -620.)
            .with_size(200., 50.);
        let button = button_builder.build_from_world(world);
        self.ui_buttons.insert(button, Buttons::Login);


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

    fn register(&self, world: &mut World) -> SimpleTrans {
        let mut ui_text_storage = world.write_storage::<UiText>();
        ui_text_storage.get_mut(*self.ui_texts.get(&Texts::Notice).unwrap()).unwrap().text = "Request register to server".to_string();

        let mut map = HashMap::new();
        map.insert("username", &ui_text_storage.get(*self.ui_texts.get(&Texts::Username).unwrap()).unwrap().text);
        map.insert("password", &ui_text_storage.get(*self.ui_texts.get(&Texts::Password).unwrap()).unwrap().text);

        let config = world.read_resource::<Request>();
        let uri = format!("{}{}", config.api_url, "/users/sign_up");

        let resp = reqwest::Client::new()
            .post(&uri)
            .json(&map)
            .send();

        let notice = match resp {
            Ok(resp) => {
                if resp.status().is_success() {
                    "Register Success"
                } else if resp.status().is_server_error() {
                    "Server is maintenance"
                } else {
                    "Username already used"
                }
            },
            Err(_) => "Server is maintenance"
        };
        ui_text_storage.get_mut(*self.ui_texts.get(&Texts::Notice).unwrap()).unwrap().text = notice.to_string();
        Trans::None
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
                            Buttons::Login => return login(data.world),
                            Buttons::Register => return self.register(data.world)
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

fn login(world: &mut World) -> SimpleTrans {
    world.delete_all();
    Trans::Switch(Box::new({
        LoginState::new()
    }))
}
