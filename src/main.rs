extern crate amethyst;
extern crate reqwest;

use amethyst::{
    prelude::*,
    ui::{DrawUi, UiBundle},
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, DepthMode, ALPHA, ColorMask},
    core::{transform::TransformBundle},
    input::{InputBundle}
};

mod states;
mod config;
mod model;
mod general;
mod components;
mod systems;

use crate::states::auth::login::State;
use crate::systems::ws_incoming_action::WsIncomingAction as WsIncomingActionSystem;
use crate::systems::outgoing_movement::OutgoingMovement as OutgoingMovementSystem;
use crate::config::Request;

fn main() -> amethyst::Result<()> {
    // amethyst::start_logger(Default::default());

    let config = DisplayConfig::load("./config/display.ron");
    let request_config = Request::load("./config/request.ron");

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new().with_transparency(
                    ColorMask::all(),
                    ALPHA,
                    Some(DepthMode::LessEqualWrite)
                ))
                .with_pass(DrawUi::new())
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
          RenderBundle::new(pipe, Some(config))
            .with_sprite_sheet_processor()
            .with_sprite_visibility_sorting(&[])
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file("./config/input.ron")?,
        )?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(WsIncomingActionSystem, "ws_incoming_action", &[])
        .with(OutgoingMovementSystem, "movement", &[]);

    let mut game = Application::build("./", State::new())?
        .with_resource(request_config)
        .build(game_data)?;

    game.run();

    Ok(())
}


