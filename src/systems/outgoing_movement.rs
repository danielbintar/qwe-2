use amethyst::{
    ecs::{Read, System},
    input::InputHandler
};

use crate::model::ws::resource::Resource as WsClient;
use crate::model::movement::payload::RequestPayload as MovementRequestPayload;
use crate::model::ws::payload::RequestPayload as WsRequestPayload;

pub struct OutgoingMovement;

#[derive(Default)]
pub struct AllowMoving {
    pub allowed: bool
}

impl<'s> System<'s> for OutgoingMovement {
    type SystemData = (
        Read<'s, AllowMoving>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, WsClient>
    );

    fn run(&mut self, (allow_moving, input, ws_client): Self::SystemData) {
        if !allow_moving.allowed {
            return;
        }

        let x_move = input.axis_value("entity_x").unwrap();
        let y_move = input.axis_value("entity_y").unwrap();

        if x_move != 0.0 || y_move != 0.0 {
            let data = MovementRequestPayload::new(x_move, y_move);
            let payload = WsRequestPayload::Move(data);
            ws_client.tx.lock().unwrap().send(serde_json::to_string(&payload).unwrap()).unwrap();
        }
    }
}
