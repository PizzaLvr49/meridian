use bevy::asset::LoadContext;
use bevy::asset::io::Reader;
use bevy::{asset::AssetLoader, prelude::*};
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoadingState>();
        app.init_asset::<PlayerAsset>();
        app.register_asset_loader(PlayerAssetLoader);
        app.add_loading_state(
            LoadingState::new(AssetLoadingState::Loading)
                .continue_to_state(AssetLoadingState::Finished)
                .load_collection::<ImageAssets>(),
        );
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PlayerMetadata {
    pub name: String,
    pub health: f32,
    pub speed: f32,
}

#[derive(Asset, TypePath)]
pub struct PlayerAsset {
    pub image: Handle<Image>,
    pub metadata: PlayerMetadata,
}

struct PlayerAssetLoader;

impl AssetLoader for PlayerAssetLoader {
    type Asset = PlayerAsset;
    type Settings = ();
    type Error = std::io::Error;

    fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext,
    ) -> impl Future<Output = Result<Self::Asset, Self::Error>> + Send {
        async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let metadata: PlayerMetadata = serde_json::from_slice(&bytes)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

            let image_path = load_context.path().with_extension("png");
            let image = load_context.load(image_path);

            Ok(PlayerAsset { image, metadata })
        }
    }

    fn extensions(&self) -> &[&str] {
        &["meta"]
    }
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "sprites/player.meta")]
    pub player: Handle<PlayerAsset>,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    Finished,
}
