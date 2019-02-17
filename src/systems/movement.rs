use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler
};

use crate::components::player::Player;
use crate::model::movement::resource::Resource as MovementClient;
use crate::model::character::{Character, CharacterPosition};
use crate::model::movement::payload::RequestPayload as RequestPayload;

use std::default::Default;

pub struct Movement;

impl<'s> System<'s> for Movement {
    type SystemData = (
        ReadStorage<'s, Player>,
        Read<'s, Character>,
        Read<'s, MovementClient>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (players, character, movement_client, mut transforms, input): Self::SystemData) {
        let x_move = input.axis_value("entity_x").unwrap();
        let y_move = input.axis_value("entity_y").unwrap();

        if x_move != 0.0 || y_move != 0.0 {
            for (_, transform) in (&players, &mut transforms).join() {
                let payload = RequestPayload {
                    id: character.get_id(),
                    x: x_move as isize,
                    y: y_move as isize
                };

                movement_client.tx.lock().unwrap().send(serde_json::to_string(&payload).unwrap()).unwrap();
            }
        }

        let received = movement_client.rx.lock().unwrap().try_recv();
        match received {
            Ok(msg) => {
                let position: CharacterPosition = serde_json::from_str(&msg).unwrap();
                for (_, transform) in (&players, &mut transforms).join() {
                    transform.set_x(position.get_x() as f32 * 10.0);
                    transform.set_y(position.get_y() as f32 * 10.0);
                }
            },
            Err(_) => {}
        }
    }
}
