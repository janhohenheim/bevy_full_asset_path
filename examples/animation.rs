use bevy::prelude::*;
use bevy_fix_gltf_coordinate_system::FixGltfCoordinateSystemPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FixGltfCoordinateSystemPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, play_animation)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 3.0, 3.0).looking_at(Vec3::Y * 0.5, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::IDENTITY.looking_to(Vec3::new(-0.5, -0.5, -1.0), Vec3::Y),
    ));

    commands.spawn(SceneRoot(
        asset_server.load("AnimationLibrary_Godot_Standard.glb#Scene0"),
    ));
}

fn play_animation(
    mut query: Query<(Entity, &mut AnimationPlayer)>,
    mut played: Local<bool>,
    mut commands: Commands,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
) {
    if *played {
        return;
    }
    for (entity, mut animation_player) in &mut query {
        *played = true;
        let animation = asset_server.load("AnimationLibrary_Godot_Standard.glb#Animation1");
        let (animation_graph, node_index) = AnimationGraph::from_clip(animation);
        let handle = animation_graphs.add(animation_graph);
        commands.entity(entity).insert(AnimationGraphHandle(handle));
        info!("Playing animation");
        animation_player.play(node_index).repeat();
    }
}
