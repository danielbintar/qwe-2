use amethyst::prelude::*;
use amethyst::assets::{Loader};
use amethyst::ui::{UiTransform, Anchor, UiText, TtfFormat};

pub struct State {
}

impl State {
    pub fn new() -> Self {
        Self {
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
}

impl SimpleState for State {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.initialize_ui(world);
    }
}
