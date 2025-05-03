#![warn(missing_docs)]
#![allow(clippy::type_complexity)]
#![doc = include_str!("../readme.md")]

use std::{f32::consts::PI, ffi::OsStr};

use bevy::{prelude::*, scene::SceneInstanceReady};

/// Everything you need to fix the coordinate system of glTF files.
pub mod prelude {
    pub use super::{DoNotFixGltfCoordinateSystem, FixGltfCoordinateSystemPlugin};
}

/// The plugin to fix the coordinate system of glTFs files.
/// Simply add it to your app to fix all glTFs.
///
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use bevy_fix_gltf_coordinate_system::prelude::*;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugins(FixGltfCoordinateSystemPlugin);
/// ```
pub struct FixGltfCoordinateSystemPlugin;
impl Plugin for FixGltfCoordinateSystemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DoNotFixGltfCoordinateSystem>();
        app.add_observer(fix_gltf_coordinates);
    }
}

/// Add this component to a scene to prevent it from being fixed.
/// The resulting scene will use the glTF coordinate system unchanged, i.e. what used to be the +Z axis stays the +Z axis.
#[derive(Component, Debug, Default, Eq, PartialEq, Hash, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct DoNotFixGltfCoordinateSystem;

fn fix_gltf_coordinates(
    trigger: Trigger<SceneInstanceReady>,
    q_scene_root: Query<(&SceneRoot, &Children), Without<DoNotFixGltfCoordinateSystem>>,
    mut q_transform: Query<&mut Transform>,
) {
    let scene_entity = trigger.target();
    let Ok((scene_root, children)) = q_scene_root.get(scene_entity) else {
        return;
    };

    let Some(asset_path) = scene_root.0.path() else {
        return;
    };

    let Some(extension) = asset_path.path().extension().and_then(OsStr::to_str) else {
        return;
    };

    const GLTF_EXTENSIONS: [&str; 2] = ["glb", "gltf"];
    if !GLTF_EXTENSIONS.contains(&extension) {
        return;
    }

    let mut iter = q_transform.iter_many_mut(children);
    while let Some(mut transform) = iter.fetch_next() {
        transform.rotate_y(PI);
    }
}
