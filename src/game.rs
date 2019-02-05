use amethyst::prelude::*;
use amethyst::assets::{Loader};
use amethyst::ui::{UiTransform, Anchor, UiText, TtfFormat, TextEditing, UiButtonBuilder, UiEventType::Click};

use specs::Entity;

use std::collections::HashMap;

pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialize_ui(world);
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Ui(x) => match x.event_type {
                Click => {
                    let world = data.world;
                    let ids = world.read_resource::<Ids>();
                    let mut ui_text_storage = world.write_storage::<UiText>();
                    ui_text_storage.get_mut(*ids.ids.get("notice").unwrap()).unwrap().text = "Request login to server".to_string();
                    println!("LOGINNNN")
                },
                _ => (),
            },
            _ => (),
        }
        Trans::None
    }
}

struct Ids {
    ids: HashMap<String, Entity>
}

fn initialize_ui(world: &mut World) {
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


    let button_builder = UiButtonBuilder::new("login_button", "LOGIN")
        .with_anchor(Anchor::TopMiddle)
        .with_position(0., -500.);
    button_builder.build_from_world(world);


    let transform = UiTransform::new(
        "notice".to_string(), Anchor::TopMiddle,
        0., -600., 1., 400., 50., 0,
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

    let mut ids = HashMap::new();
    ids.insert("notice".to_string(), notice);
    world.add_resource(Ids{ids: ids});


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

    world
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
    let mut ui_text_storage = world.write_storage::<UiText>();
    ui_text_storage.get_mut(password).unwrap().password = true;
}
