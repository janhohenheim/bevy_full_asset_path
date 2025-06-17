#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]
use bevy::prelude::*;

pub use path_provider::FullAssetPathProvider;

mod path_provider;

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

        app.add_plugins(path_provider::plugin);
    }
}
