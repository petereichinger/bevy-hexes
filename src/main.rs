mod coordinates;
mod hex_world;
mod input;
mod mesh_generation;

use bevy_rapier3d::prelude::*;
use hex_world::HexWorld;
use input::camera_control_plugin::CameraControlPlugin;

use bevy::{prelude::*, render::texture::ImageSettings};

fn main() {
    let window_descriptor = WindowDescriptor {
        title: "bevy-hexes".into(),
        position: WindowPosition::Centered(MonitorSelection::Number(1)),
        ..Default::default()
    };

    App::new()
        .insert_resource(window_descriptor)
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraControlPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(create_plane_and_light)
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(HexWorld)
        .run()
}

fn create_plane_and_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(20., 16.0, 0.),
        ..default()
    });

    // ground plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(shape::Plane { size: 50. }.into()),
        material: materials.add(Color::GOLD.into()),
        ..default()
    });
}
