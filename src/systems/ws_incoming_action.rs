use amethyst::{
    core::Transform,
    assets::{AssetStorage, Loader},
    ecs::{Join, Read, System, WriteStorage, Entities, ReadExpect},
    input::InputHandler,
    renderer::{SpriteSheet, Texture, SpriteRender},
    ui::UiText
};

use crate::components::chat::Show as ShowChat;
use crate::components::player::Player;
use crate::model::ws::resource::Resource as WsClient;
use crate::model::character::{Character, CharacterPosition};
use crate::model::ws::payload::{RequestPayload, ResponsePayload};

use crate::general;

pub struct WsIncomingAction;

impl<'s> System<'s> for WsIncomingAction {
    type SystemData = (
        Read<'s, WsClient>,
        WriteStorage<'s, ShowChat>,
        WriteStorage<'s, UiText>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, (ws_client, mut chat_shows, mut ui_texts,
        mut players, mut transforms): Self::SystemData) {
        let received = ws_client .rx.lock().unwrap().try_recv();
        match received {
            Ok(msg) => {
                let ws_payload: ResponsePayload = serde_json::from_str(&msg).unwrap();
                match ws_payload {
                    ResponsePayload::Chat(payload) => {
                        for (chat_show, ui_text) in (&chat_shows, &mut ui_texts).join() {
                            ui_text.text.push_str(&payload.get_full_message());
                            ui_text.text.push_str("\n");
                        }
                    },
                    ResponsePayload::Move(payload) => {
                        for (player, transform) in (&players, &mut transforms).join() {
                            if player.get_id() == payload.get_id() {
                                transform.set_x((payload.get_x() * general::GRID_SCALE_X) as f32);
                                transform.set_y((payload.get_y() * general::GRID_SCALE_Y) as f32);
                                break;
                            }
                        }
                    }
                }
            },
            Err(_) => {}
        }
    }
}
