pub mod auth;
pub mod character;
pub mod town;
pub mod has_chat;
pub mod has_characters;
pub mod region;
pub mod battle;

use amethyst::{
    prelude::*,
    core::Transform,
    assets::{AssetStorage, Loader},
    renderer::{
        Camera, Projection,
        PngFormat, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata
    }
};

fn load_sprite_sheet(world: &World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
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

fn init_default_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -250.0, 250.0, -250.0, 250.0,
        )))
        .with(transform)
        .build();
}
