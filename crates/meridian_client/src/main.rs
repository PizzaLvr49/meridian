#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use bevy_panic::PanicHandler;
use meridian_assets_handler::{AssetLoadingState, AssetPlugin, ImageAssets, TileAsset, TileAssets};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                name: Some("Meridian".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PanicHandler::default())
        .add_plugins(AssetPlugin)
        .add_systems(
            OnEnter(AssetLoadingState::Finished),
            (spawn_player, test_tile),
        )
        .run();
}

fn spawn_player(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn(Sprite::from_image(image_assets.player.clone()));
}

fn test_tile(tile_assets: Res<TileAssets>, tiles: Res<Assets<TileAsset>>) {
    if let Some(tile) = tiles.get(&tile_assets.conveyer) {
        info!("\n{:#?}", tile.data);
    } else {
        info!("Tile asset not loaded yet");
    }
}
