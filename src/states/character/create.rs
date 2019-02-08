use amethyst::prelude::*;
use amethyst::assets::{Loader};
use amethyst::ui::{UiTransform, Anchor, UiText, TtfFormat, UiButtonBuilder, UiEventType::Click};

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

pub struct State {
    ui_buttons: HashMap<Entity, Buttons>
}

impl State {
    pub fn new() -> Self {
        let btn_count = 2;

        Self {
            ui_buttons: HashMap::with_capacity(btn_count)
        }
    }

    fn initialize_ui(&mut self, world: &mut World) {
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
