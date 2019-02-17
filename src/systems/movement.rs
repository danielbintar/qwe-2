use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler
};

use crate::components::player::Player;
use crate::model::movement::resource::Resource as MovementClient;
use crate::model::character::{Character, CharacterPosition};
use crate::model::movement::payload::RequestPayload as RequestPayload;


use crate::general;

pub struct Movement {
    counter: usize
}

impl Movement {
    pub fn new() -> Self {
        Self {
            counter: 0
        }
    }
}

impl<'s> System<'s> for Movement {
    type SystemData = (
        ReadStorage<'s, Player>,
        Read<'s, Character>,
        Read<'s, MovementClient>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (players, character, movement_client, mut transforms, input): Self::SystemData) {
        let received = movement_client.rx.lock().unwrap().try_recv();
        match received {
            Ok(msg) => {
                let msgs: Vec<&str> = msg.split("\n").collect();
                for decoded_position in &msgs {
                    let position: CharacterPosition = serde_json::from_str(&decoded_position).unwrap();
                    for (player, transform) in (&players, &mut transforms).join() {
                        if player.get_id() == position.get_id() {
                            transform.set_x((position.get_x() * general::GRID_SCALE_X) as f32);
                            transform.set_y((position.get_y() * general::GRID_SCALE_Y) as f32);
                        }
                    }
                }
            },
            Err(_) => {}
        }

        self.counter += 1;
        if self.counter < 10 {
            return;
        }
        self.counter = 0;

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
    }
}
