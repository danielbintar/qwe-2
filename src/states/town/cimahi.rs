use amethyst::{
    prelude::*,
    assets::{AssetStorage, Loader},
    core::{Parent, Transform},
    ecs::Entity,
    renderer::{
        Camera, PngFormat, Projection,
        SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
        Texture, TextureMetadata, Transparent
    }
};

use crate::components::player::Player;

pub struct State {
    
}

fn init_reference_sprite(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_x(0.0);
    transform.set_y(0.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Transparent)
        .build()
}

impl State {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl SimpleState for State {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        super::super::init_chat_ui(world);
        let player_sprite = load_sprite_sheet(world, "./resources/sprites/player.png", "./resources/sprites/player.ron");
        let _reference = init_reference_sprite(world, &player_sprite);
        let parent = init_player(world, &player_sprite);
        init_camera(world, parent);
    }
}

fn init_player(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_x(0.0);
    transform.set_y(0.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .with(Player)
        .with(Transparent)
        .build()
}

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            png_path,
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
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
