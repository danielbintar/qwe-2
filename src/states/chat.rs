use amethyst::{
    prelude::*,
    assets::{Loader},
    ecs::Entity,
    ui::{UiTransform, Anchor, UiText, TtfFormat, TextEditing, LineMode::Wrap, UiButtonBuilder, UiEventType::Click}
};

pub trait ChatState {
    fn get_chat_button(&self) -> Entity;
    fn set_chat_button(&mut self, e: Entity);

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
                120., 100., 1., 250., 400., 0
            );

        let mut chat_input_text = UiText::new(
                font.clone(),
                "a".to_string(),
                [1., 1., 1., 1.],
                20.);
        chat_input_text.line_mode = Wrap;

        world
            .create_entity()
            .with(transform)
            .with(chat_input_text)
            .with(TextEditing::new(
                100,
                [1., 1., 1., 1.],
                [0.0, 0.0, 0.0, 1.0],
                false))
            .build();


        let button_builder = UiButtonBuilder::new("chat_button", "Send")
            .with_anchor(Anchor::BottomLeft)
            .with_position(300., 100.);
        let button = button_builder.build_from_world(world);
        self.set_chat_button(button);
    }

    

    fn handle_chat(&self, world: &mut World, event: StateEvent) {
        match event {
            StateEvent::Ui(x) => match x.event_type {
                Click => {
                    if(x.target == self.get_chat_button()) {
                        let r = world.read_resource::<crate::model::chat::resource::Resource>();
                        r.tx.lock().unwrap().send("a".to_string());
                    }
                },
                _ => (),
            },
            _ => (),
        }
    }
}
