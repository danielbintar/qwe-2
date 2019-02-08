use amethyst::prelude::*;
use amethyst::assets::{Loader};
use amethyst::ui::{UiTransform, Anchor, UiText, TextEditing, TtfFormat, UiButtonBuilder, UiEventType::Click};

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
    Back
}

#[derive(PartialEq, Eq, Hash)]
enum Texts {
    Name,
    Notice
}

pub struct State {
    ui_buttons: HashMap<Entity, Buttons>,
    ui_texts: HashMap<Texts, Entity>
}

impl State {
    pub fn new() -> Self {
        let btn_count = 2;
        let text_count = 2;

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
                            Buttons::Back => return self.back(data.world),
                            _ => ()
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
