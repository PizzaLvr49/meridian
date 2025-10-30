use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Next),
        );
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    Loading,
    Next,
}
