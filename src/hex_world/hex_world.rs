use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    coordinates::Axial, coordinates::Cube, coordinates::Offset,
    input::camera_control_plugin::CurrentCameraTag, mesh_generation::hex::create_hex_prism,
};

#[derive(Component)]
struct Hexagon(Cube);

#[derive(Component)]
struct Selected;

pub struct HexWorld;

impl Plugin for HexWorld {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(move_hexes)
            .add_system(select_hex)
            .add_system(log_selected_hex);
    }
}

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

    if !hex_str.is_empty() {
        info!("{}", hex_str);
    }
}
