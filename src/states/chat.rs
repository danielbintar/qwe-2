use amethyst::{
    prelude::*,
    assets::{Loader},
    ecs::Entity,
    ui::{UiTransform, Anchor, UiText, TtfFormat, TextEditing, LineMode::Wrap, UiButtonBuilder, UiEventType::Click}
};

use crate::model::chat::payload::ResponsePayload;
use crate::model::chat::payload::RequestPayload;

pub trait ChatState {
    fn get_chat_button(&self) -> Entity;
    fn set_chat_button(&mut self, e: Entity);
    fn get_chat_input(&self) -> Entity;
    fn set_chat_input(&mut self, e: Entity);
    fn get_chat_show(&self) -> Entity;
    fn set_chat_show(&mut self, e: Entity);

    fn init_chat_ui(&mut self, world: &mut World) {
        let font = world.read_resource::<Loader>().load(
                "./resources/font/square.ttf",
                TtfFormat,
                Default::default(),
                (),
                &world.read_resource(),
            );

        let transform = UiTransform::new(
                "chat".to_string(), Anchor::BottomLeft,
                120., 350., 1., 250., 400., 0
            );

        let mut chat_show_text = UiText::new(
                font.clone(),
                "".to_string(),
                [1., 1., 1., 1.],
                20.);
        chat_show_text.line_mode = Wrap;

        let chat_show = world
            .create_entity()
            .with(transform)
            .with(chat_show_text)
            .build();
        self.set_chat_show(chat_show);


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

        let chat_input = world
            .create_entity()
            .with(transform)
            .with(chat_input_text)
            .with(TextEditing::new(
                100,
                [1., 1., 1., 1.],
                [0.0, 0.0, 0.0, 1.0],
                false))
            .build();
        self.set_chat_input(chat_input);


        let button_builder = UiButtonBuilder::new("chat_button", "Send")
            .with_anchor(Anchor::BottomLeft)
            .with_position(300., 100.);
        let button = button_builder.build_from_world(world);
        self.set_chat_button(button);
    }

    fn handle_send_chat(&self, world: &mut World, event: StateEvent) {
        match event {
            StateEvent::Ui(x) => match x.event_type {
                Click => {
                    if x.target == self.get_chat_button() {
                        let ui_text_storage = world.write_storage::<UiText>();
                        let message = ui_text_storage.get(self.get_chat_input()).unwrap().text.clone();
                        let r = world.read_resource::<crate::model::chat::resource::Resource>();
                        let payload = RequestPayload::new(message);
                        r.tx.lock().unwrap().send(serde_json::to_string(&payload).unwrap()).unwrap();
                    }
                },
                _ => (),
            },
            _ => (),
        }
    }

    fn handle_receive_chat(&self, world: &mut World) {
        let r = world.read_resource::<crate::model::chat::resource::Resource>();
        let received = r.rx.lock().unwrap().try_recv();
        match received {
            Ok(msg) => {
                let payload: ResponsePayload = serde_json::from_str(&msg).unwrap();
                let mut ui_text_storage = world.write_storage::<UiText>();
                let t = ui_text_storage.get_mut(self.get_chat_show()).unwrap();
                t.text.push_str(&payload.get_full_message());
                t.text.push_str("\n");
            },
            Err(_) => {}
        }
    }
}
