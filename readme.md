# bevy_full_asset_path

[![crates.io](https://img.shields.io/crates/v/bevy_full_asset_path)](https://crates.io/crates/bevy_full_asset_path)
[![docs.rs](https://docs.rs/bevy_full_asset_path/badge.svg)](https://docs.rs/bevy_full_asset_path)

A tiny plugin that allows reading the full filesystem path of an asset loaded from disk


## Usage

Add `FullAssetPathPlugin` after `DefaultPlugins`, then use the `FullAssetPathProvider` resource to retrieve full paths:

```rust,no_run
use bevy::prelude::*;
use bevy_full_asset_path::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FullAssetPathPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(asset_server: Res<AssetServer>, full_path_provider: Res<FullAssetPathProvider>) {
    let sprite: Handle<Image> = asset_server.load("bird/icon.png");
    let asset_path = sprite.path().unwrap();
    let full_path = full_path_provider.full_asset_path(asset_path).unwrap();
    info!("Full asset path: {}", full_path.display());
}
```
