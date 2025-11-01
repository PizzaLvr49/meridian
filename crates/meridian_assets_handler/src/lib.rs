use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoadingState>();
        app.add_loading_state(
            LoadingState::new(AssetLoadingState::Loading)
                .continue_to_state(AssetLoadingState::Finished)
                .load_collection::<ImageAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "sprites/player.png")]
    pub player: Handle<Image>,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    Finished,
}
