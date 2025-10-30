#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use bevy_panic::PanicHandler;
use meridian_assets_handler::TilePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                name: Some("factory game".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PanicHandler::default())
        .add_plugins(TilePlugin)
        .run();
}
