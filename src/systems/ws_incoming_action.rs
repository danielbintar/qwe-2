use amethyst::{
    core::Transform,
    assets::{AssetStorage, Loader},
    ecs::{Join, Read, System, WriteStorage, ReadStorage, Entities, ReadExpect, Write},
    renderer::{SpriteSheet, Texture, SpriteRender},
    ui::UiText
};

use crate::{
    general,
    model::{
        action::{Action, PlayerAction},
        place::CurrentPlace,
        character::Character,
        ws::{
            payload::ResponsePayload,
            resource::Resource as WsClient
        }
    },
    components::{
        chat::Show as ShowChat,
        player::Player
    }
};

pub struct WsIncomingAction;

impl<'s> System<'s> for WsIncomingAction {
    type SystemData = (
        Read<'s, WsClient>,
        ReadStorage<'s, ShowChat>,
        WriteStorage<'s, UiText>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        ReadExpect<'s, Loader>,
        Read<'s, AssetStorage<Texture>>,
        Read<'s, AssetStorage<SpriteSheet>>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Character>,
        Write<'s, Action>,
        Read<'s, CurrentPlace>
    );

    fn run(&mut self, (ws_client, chat_shows, mut ui_texts,
        mut players, mut transforms,
        entities, loader, texture_storage, sprite_sheet_storage, mut sprite_render_storage,
        character, mut action, current_place): Self::SystemData) {
        let received = ws_client .rx.lock().unwrap().try_recv();
        match received {
            Ok(msg) => {
                let ws_payload: ResponsePayload = serde_json::from_str(&msg).unwrap();
                match ws_payload {
                    ResponsePayload::Ping => (),
                    ResponsePayload::LeaveRegion(payload) => {
                        if payload.get_id() == character.get_id() {
                            action.action = Some(PlayerAction::LeaveRegion);
                            return;
                        }
                        for (player, entity) in (&players, &entities).join() {
                            if player.get_id() == payload.get_id() {
                                entities.delete(entity).unwrap();
                                break;
                            }
                        }
                    },
                    ResponsePayload::LeaveTown(payload) => {
                        if payload.get_id() == character.get_id() {
                            action.action = Some(PlayerAction::LeaveTown);
                            return;
                        }
                        for (player, entity) in (&players, &entities).join() {
                            if player.get_id() == payload.get_id() {
                                entities.delete(entity).unwrap();
                                break;
                            }
                        }
                    },
                    ResponsePayload::Logout(payload) => {
                        for (player, entity) in (&players, &entities).join() {
                            if player.get_id() == payload.get_id() {
                                entities.delete(entity).unwrap();
                                break;
                            }
                        }
                    },
                    ResponsePayload::Chat(payload) => {
                        for (_chat_show, ui_text) in (&chat_shows, &mut ui_texts).join() {
                            ui_text.text.push_str(&payload.get_full_message());
                            ui_text.text.push_str("\n");
                        }
                    },
                    ResponsePayload::Move(payload) => {
                        let active_place = payload.get_active_place();
                        if active_place != current_place.place.clone().unwrap() {
                            return;
                        }
                        let mut found = false;
                        for (player, transform) in (&players, &mut transforms).join() {
                            if player.get_id() == payload.get_id() {
                                found = true;
                                transform.set_x((payload.get_x() * general::GRID_SCALE_X) as f32);
                                transform.set_y((payload.get_y() * general::GRID_SCALE_Y) as f32);
                                break;
                            }
                        }

                        if !found {
                            let handler = super::load_sprite_sheet(loader, texture_storage, sprite_sheet_storage);

                            let sprite = SpriteRender {
                                sprite_sheet: handler.clone(),
                                sprite_number: 0,
                            };

                            let mut transform = Transform::default();
                            transform.set_x((payload.get_x() * general::GRID_SCALE_X) as f32);
                            transform.set_y((payload.get_y() * general::GRID_SCALE_Y) as f32);
                            entities.build_entity()
                                .with(transform, &mut transforms)
                                .with(Player::new(payload.get_id()), &mut players)
                                .with(sprite, &mut sprite_render_storage)
                                .build();
                        }
                    }
                }
            },
            Err(_) => {}
        }
    }
}
