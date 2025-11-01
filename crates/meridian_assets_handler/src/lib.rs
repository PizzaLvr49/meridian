use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AssetLoadingState>();
        app.init_asset::<TileAsset>();
        app.register_asset_loader(TileAssetLoader);
        app.add_loading_state(
            LoadingState::new(AssetLoadingState::Loading)
                .continue_to_state(AssetLoadingState::Finished)
                .load_collection::<ImageAssets>()
                .load_collection::<TileAssets>(),
        );
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct TileData {
    pub name: String,
    pub health: i32,
    pub size: UVec2,
}

#[derive(Asset, TypePath)]
pub struct TileAsset {
    pub image: Handle<Image>,
    pub data: TileData,
}

pub struct TileAssetLoader;

impl AssetLoader for TileAssetLoader {
    type Asset = TileAsset;
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
            let data: TileData = ron::de::from_bytes(&bytes)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

            let image_path = load_context.path().with_extension("png");
            let image = load_context.load(image_path);

            Ok(TileAsset { image, data })
        }
    }

    fn extensions(&self) -> &[&str] {
        &["tile.ron"]
    }
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "sprites/player.png")]
    pub player: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct TileAssets {
    #[asset(path = "tiles/conveyer.tile.ron")]
    pub conveyer: Handle<TileAsset>,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    Finished,
}
