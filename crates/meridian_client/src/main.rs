#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use bevy_panic::PanicHandler;
use meridian_assets_handler::{AssetLoadingState, AssetPlugin, ImageAssets, PlayerAsset};

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
        .add_systems(OnEnter(AssetLoadingState::Finished), spawn_player)
        .run();
}

fn spawn_player(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    player_assets: Res<Assets<PlayerAsset>>,
) {
    if let Some(player_asset) = player_assets.get(&image_assets.player) {
        commands.spawn(Sprite::from_image(player_asset.image.clone()));

        println!("Player name: {}", player_asset.metadata.name);
        println!("Player health: {}", player_asset.metadata.health);
        println!("Player speed: {}", player_asset.metadata.speed);
    }
}
