use amethyst::{
    core::Transform,
    assets::{AssetStorage, Loader},
    ecs::{Join, Read, System, WriteStorage, Entities, ReadExpect},
    input::InputHandler,
    renderer::{SpriteSheet, Texture, SpriteRender},
    ui::UiText
};

use crate::components::chat::Show as ShowChat;
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
    );

    fn run(&mut self, (ws_client, mut chat_shows, mut ui_texts): Self::SystemData) {
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
                    _ => ()
                }
            },
            Err(_) => {}
        }
    }
}
