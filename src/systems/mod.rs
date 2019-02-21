pub mod movement;

use amethyst::{
    core::Transform,
    assets::{AssetStorage, Loader, Handle},
    ecs::{Join, Read, ReadStorage, System, WriteStorage, Write, ReadExpect},
    input::InputHandler,
    renderer::{
        PngFormat, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata
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
