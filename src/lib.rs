#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]
use std::path::PathBuf;

use bevy::{
    asset::{
        AssetPath,
        io::{AssetSourceId, file::FileAssetReader},
    },
    prelude::*,
};

/// Everything you need to get started.
pub mod prelude {
    pub use super::{FullAssetPathPlugin, FullAssetPathProvider};
}

/// The plugin that allows access to [`FullAssetPathProvider`] on supported platforms.
///
/// Requires access to the [`AssetPlugin`] to work, which is commonly added as part of the [`DefaultPlugins`].
///
/// # Example
///
/// ```rust,ignore
/// # use bevy::prelude::*;
/// # use bevy_full_asset_path::prelude::*;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(FullAssetPathPlugin::default())
///     .run();
/// ```
#[derive(Default)]
#[non_exhaustive]
pub struct FullAssetPathPlugin;

impl Plugin for FullAssetPathPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FullAssetPathProvider>();
        #[cfg(target_arch = "wasm32")]
        {
            return;
        }
        let asset_plugin = get_asset_plugin(app);
        let path_str = asset_plugin.file_path.clone();
        let path = PathBuf::from(path_str);
        app.insert_resource(FullAssetPathProvider {
            relative_assets_dir: path,
        });
    }
}

fn get_asset_plugin(app: &App) -> &AssetPlugin {
    let asset_plugins: Vec<&AssetPlugin> = app.get_added_plugins();
    asset_plugins.into_iter().next().expect(ASSET_ERROR)
}

const ASSET_ERROR: &str = "bevy_full_asset_path requires access to the Bevy asset plugin. \
    Please add `FullAssetPathPlugin` after `AssetPlugin`, which is commonly added as part of the `DefaultPlugins`";

/// If we're using a [`FileAssetReader`] to read an asset,
/// this resource will tell us the full path used to load something from a given [`AssetPath`].
///
/// Only available on platforms where [`FileAssetReader`] is also available.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct FullAssetPathProvider {
    relative_assets_dir: PathBuf,
}

impl FullAssetPathProvider {
    /// Get the full path used to load an asset from a given [`AssetPath`].
    ///
    /// Returns `Err` if the asset was not loaded from disk.
    pub fn full_asset_path(&self, asset_path: &AssetPath) -> Result<PathBuf, FullAssetPathError> {
        match asset_path.source() {
            AssetSourceId::Default => {}
            AssetSourceId::Name(name) => {
                return Err(FullAssetPathError::NonDefaultSource(name.to_string()));
            }
        };
        let base_path = FileAssetReader::get_base_path();
        let relative_asset_path = self.relative_assets_dir.as_path();
        let full_path = base_path.join(relative_asset_path).join(asset_path.path());
        Ok(full_path)
    }
}

/// Error returned by [`FullAssetPathProvider::full_asset_path`].
#[derive(Debug, thiserror::Error)]
pub enum FullAssetPathError {
    #[error("Asset path is not coming from the default source: {0}")]
    /// The requested asset was not loaded from the default source.
    NonDefaultSource(String),
}
