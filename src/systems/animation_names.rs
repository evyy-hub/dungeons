use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

// 1. The data type itself.
// It just wraps a map from animation name -> node index.
#[derive(Asset, TypePath, Deserialize)]
pub struct AnimationNames {
    names: HashMap<String, u32>,
}

impl AnimationNames {
    pub fn try_get(&self, name: &str) -> Option<AnimationNodeIndex> {
        self.names
            .get(name)
            .map(|&idx| AnimationNodeIndex::new(idx as usize))
    }
}

// 2. The loader: tells Bevy how to turn the raw bytes of a
// ".animnames.ron" file into an AnimationNames value.
#[derive(Default, TypePath)]
pub struct AnimationNamesLoader;

impl AssetLoader for AnimationNamesLoader {
    type Asset = AnimationNames;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let names: HashMap<String, u32> = ron::de::from_bytes(&bytes)?;
        Ok(AnimationNames { names })
    }

    fn extensions(&self) -> &[&str] {
        &["animnames.ron"]
    }
}

// 3. The plugin: registers the asset type and its loader with the App.
pub struct AnimationNamesPlugin;

impl Plugin for AnimationNamesPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AnimationNames>()
            .init_asset_loader::<AnimationNamesLoader>();
    }
}
