extern crate amethyst;
extern crate specs;
extern crate reqwest;

use amethyst::prelude::*;
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage};
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;

mod states;
mod config;
mod model;

use crate::states::login::State;
use crate::config::Request;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let config = DisplayConfig::load("./config/display.ron");
    let request_config = Request::load("./config/request.ron");

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new())
                .with_pass(DrawUi::new()),
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
          RenderBundle::new(pipe, Some(config))
            .with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<String, String>::new())?
        .with_bundle(UiBundle::<String, String>::new())?;

    let mut game = Application::build("./", State::new())?
        .with_resource(request_config)
        .build(game_data)?;

    game.run();

    Ok(())
}


