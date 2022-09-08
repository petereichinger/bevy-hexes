mod coordinates;
mod input;
mod mesh_generation;

use bevy_rapier3d::prelude::*;
use coordinates::{Axial, Cube, Offset};
use input::camera_control_plugin::{CameraControlPlugin, CurrentCameraTag};
use mesh_generation::hex::create_hex_prism;

use bevy::{prelude::*, render::texture::ImageSettings};

#[derive(Component)]
struct Hexagon(Cube);

#[derive(Component)]
struct Selected;

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
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(move_hexes)
        .add_system(select_hex)
        .add_system(log_selected_hex)
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
// #[derive(Component)]
// struct Shape;

fn move_hexes(time: Res<Time>, mut query: Query<(&mut Transform, &Hexagon)>) {
    let origin = Default::default();
    let startup_time = 1.5 * time.time_since_startup().as_secs_f32();
    for (mut transform, hex) in query.iter_mut() {
        let distance = hex.0.distance_to(origin) as f32;

        transform.translation.y = 1. + 0.25 * (distance + -startup_time).sin();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        // base_color_texture: Some(images.add(uv_debug_texture())),
        base_color: Color::rgb_u8(0, 98, 105),
        metallic: 0.8,
        ..default()
    });

    let mesh = meshes.add(create_hex_prism(0.49, 0.25).into());

    for x in -25..=25 {
        for y in -25..=25 {
            let axial: Axial = Offset::new(x, y).into();
            commands
                .spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    material: material.clone(),
                    transform: Transform {
                        translation: axial.into(),
                        ..default()
                    },
                    ..default()
                })
                .insert(Hexagon(axial.into()))
                .insert(RigidBody::KinematicPositionBased)
                .insert(
                    Collider::from_bevy_mesh(
                        Assets::get(&meshes, &mesh).unwrap(),
                        &ComputedColliderShape::TriMesh,
                    )
                    .unwrap(),
                );
        }
    }

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

fn select_hex(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    camera_query: Query<(&GlobalTransform, &Camera), With<CurrentCameraTag>>,
    hexagon_query: Query<(Entity, &Hexagon, Option<&Selected>)>,
    btn: Res<Input<MouseButton>>,
) {
    if !btn.just_pressed(MouseButton::Left) {
        return;
    }
    let (transform, camera) = camera_query.single();

    let ray_origin = camera
        .ndc_to_world(
            transform,
            Vec3 {
                x: 0.,
                y: 0.,
                z: 1.,
            },
        )
        .unwrap();
    let ray_dir = transform.forward();
    let max_toi = 100.;
    let solid = true;
    if let Some((entity, _toi)) =
        rapier_context.cast_ray(ray_origin, ray_dir, max_toi, solid, Default::default())
    {
        if let Some((entity, hex, selected)) = hexagon_query
            .iter()
            .find(|(ent, _hex, _selected)| ent.id() == entity.id())
        {
            let coord = Offset::from(hex.0);
            info!("{}", coord);

            let mut entitycommands = commands.entity(entity);
            if let Some(Selected) = selected {
                entitycommands.remove::<Selected>()
            } else {
                entitycommands.insert(Selected)
            };
        }
    }
}

fn log_selected_hex(hexagon_query: Query<(&Hexagon, &Selected)>) {
    let hex_str: String = hexagon_query
        .iter()
        .map(|(hex, _)| format!("{} ", hex.0))
        .collect();

    info!("{}", hex_str);
}
