use amethyst::{
    prelude::*,
    core::{Transform, EventReader},
    assets::{AssetStorage, Loader, Handle},
    ecs::{Join, Read, ReadStorage, System, WriteStorage, Write, Entities, ReadExpect, Resources},
    shred::{RunNow, RunWithPool, SetupHandler, SystemData},
    input::InputHandler,
    renderer::{
        PngFormat, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata, SpriteRender
    }
};

use crate::components::player::Player;
use crate::model::movement::resource::Resource as MovementClient;
use crate::model::character::{Character, CharacterPosition};
use crate::model::movement::payload::RequestPayload as RequestPayload;

use crate::general;

pub struct Movement {

}

#[derive(Default)]
pub struct AllowMoving {
    pub allowed: bool
}

impl Movement {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl<'s> System<'s> for Movement {
    type SystemData = (
        WriteStorage<'s, Player>,
        Read<'s, Character>,
        Read<'s, MovementClient>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, AllowMoving>,
        Entities<'s>,
        ReadExpect<'s, Loader>,
        Read<'s, AssetStorage<Texture>>,
        Read<'s, AssetStorage<SpriteSheet>>,
        WriteStorage<'s, SpriteRender>
    );

    fn run(&mut self, (mut players, character, movement_client, mut transforms,
        input, allow_moving, entities, loader, texture_storage,
        sprite_sheet_storage, mut sprite_render_storage): Self::SystemData) {
        let received = movement_client.rx.lock().unwrap().try_recv();
        match received {
            Ok(msg) => {
                let msgs: Vec<&str> = msg.split("\n").collect();
                for decoded_position in &msgs {
                    let position: CharacterPosition = serde_json::from_str(&decoded_position).unwrap();
                    let mut found = false;
                    for (player, transform) in (&players, &mut transforms).join() {
                        if player.get_id() == position.get_id() {
                            found = true;
                            transform.set_x((position.get_x() * general::GRID_SCALE_X) as f32);
                            transform.set_y((position.get_y() * general::GRID_SCALE_Y) as f32);
                        }
                    }

                    if !found {
                        let handler = super::load_sprite_sheet(loader, texture_storage, sprite_sheet_storage);
                        let sprite = SpriteRender {
                            sprite_sheet: handler.clone(),
                            sprite_number: 0,
                        };
                        let mut transform = Transform::default();
                        transform.set_x((position.get_x() * general::GRID_SCALE_X) as f32);
                        transform.set_y((position.get_y() * general::GRID_SCALE_Y) as f32);
                        entities.build_entity()
                            .with(transform, &mut transforms)
                            .with(Player::new(position.get_id()), &mut players)
                            .with(sprite, &mut sprite_render_storage)
                            .build();
                        break;
                    }
                }
            },
            Err(_) => {}
        }

        if !allow_moving.allowed {
            return;
        }

        let x_move = input.axis_value("entity_x").unwrap();
        let y_move = input.axis_value("entity_y").unwrap();

        if x_move != 0.0 || y_move != 0.0 {
            let payload = RequestPayload {
                id: character.get_id(),
                x: x_move as isize,
                y: y_move as isize
            };

            movement_client.tx.lock().unwrap().send(serde_json::to_string(&payload).unwrap()).unwrap();
        }
    }
}
