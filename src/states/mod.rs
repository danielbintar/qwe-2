pub mod auth;
pub mod character;
pub mod town;
pub mod region;
pub mod has_chat;
pub mod has_characters;

use amethyst::{
    prelude::*,
    assets::{AssetStorage, Loader},
    renderer::{
        PngFormat, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata
    }
};

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

#[derive(Clone)]
pub enum PlayerAction {
    LeaveTown
}
