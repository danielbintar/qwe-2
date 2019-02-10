use amethyst::{
    prelude::*,
    assets::{Loader},
    ui::{UiTransform, Anchor, UiText, TtfFormat, TextEditing, LineMode::Wrap}
};

pub mod auth;
pub mod character;
pub mod town;

fn init_chat_ui(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
            "./resources/font/square.ttf",
            TtfFormat,
            Default::default(),
            (),
            &world.read_resource(),
        );

    let transform = UiTransform::new(
            "chat".to_string(), Anchor::BottomLeft,
            120., 100., 1., 250., 400., 0
        );

    let mut chat_input_text = UiText::new(
            font.clone(),
            "a".to_string(),
            [1., 1., 1., 1.],
            20.);
    chat_input_text.line_mode = Wrap;

	world
        .create_entity()
        .with(transform)
        .with(chat_input_text)
        .with(TextEditing::new(
            100,
            [1., 1., 1., 1.],
            [0.0, 0.0, 0.0, 1.0],
            false))
        .build();


}
