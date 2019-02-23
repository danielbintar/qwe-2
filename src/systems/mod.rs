pub mod movement;
pub mod ws_incoming_action;

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    ecs::{Read, ReadExpect},
    renderer::{
        PngFormat, SpriteSheet, SpriteSheetFormat,
        Texture, TextureMetadata
    }
};

fn load_sprite_sheet(loader: ReadExpect<Loader>, texture_storage: Read<AssetStorage<Texture>>, sprite_sheet_store: Read<AssetStorage<SpriteSheet>>) -> Handle<SpriteSheet> {
    let texture_handle = {
        loader.load(
            "./resources/sprites/player.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    loader.load(
        "./resources/sprites/player.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}
