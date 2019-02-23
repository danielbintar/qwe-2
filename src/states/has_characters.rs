use amethyst::{
    prelude::*,
    ecs::Entity,
    core::{Parent, Transform},
    renderer::{
        Camera, Projection,
        SpriteRender, SpriteSheetHandle, Transparent
    }
};

use crate::{
    general,
    components::player::Player,
    model::character::{
        Character,
        CharacterPosition
    }
};

pub trait HasCharacters {
    fn set_characters_position(&mut self, c: Vec<CharacterPosition>);
    fn get_characters_position(&self) -> Vec<CharacterPosition>;

    fn init_characters_ui(&mut self, world: &mut World) {
        let characters = self.get_characters_position();

        let current_character_id = world.read_resource::<Character>().get_id();
        let mut parent: Option<Entity> = None;

        for character in &characters {
            let player_sprite = super::load_sprite_sheet(world, "./resources/sprites/player.png", "./resources/sprites/player.ron");
            let player = init_player(world, &player_sprite, character);
            if character.get_id() == current_character_id {
                parent = Some(player);
            }
        }
        init_camera(world, parent.unwrap());
    }
}

fn init_camera(world: &mut World, parent: Entity) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -250.0, 250.0, -250.0, 250.0,
        )))
        .with(Parent { entity: parent })
        .with(transform)
        .build();
}

fn init_player(world: &mut World, sprite_sheet: &SpriteSheetHandle, character: &CharacterPosition) -> Entity {
    let mut transform = Transform::default();
    transform.set_x((character.get_x() * general::GRID_SCALE_X) as f32);
    transform.set_y((character.get_y() * general::GRID_SCALE_Y) as f32);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Player::new(character.get_id()))
        .with(Transparent)
        .build()
}
