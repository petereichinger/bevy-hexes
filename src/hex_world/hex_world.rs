use std::{cmp::Ordering, collections::HashMap};

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    coordinates::Axial, coordinates::Cube, coordinates::Offset,
    input::camera_control_plugin::CurrentCameraTag, mesh_generation::hex::create_hex_prism,
};

#[derive(Component)]
struct Hexagon(Cube);

#[derive(Component)]
struct Energy(f32);

pub struct HexWorld;

impl Plugin for HexWorld {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(select_hex)
            // .add_system(reduce_energy)
            .add_system(distribute_energy)
            .add_system(move_hexes.after(distribute_energy));
    }
}

fn move_hexes(time: Res<Time>, mut query: Query<(&mut Transform, &Energy)>) {
    // let origin = Default::default();
    // let startup_time = 1.5 * time.time_since_startup().as_secs_f32();
    // for (mut transform, hex) in query.iter_mut() {
    //     let distance = hex.0.distance_to(origin) as f32;

    //     transform.translation.y = 1. + 0.25 * (distance + -startup_time).sin();
    // }

    for (mut transform, energy) in query.iter_mut() {
        transform.translation.y = energy.0;
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
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
                )
                .insert(Energy(0.));
        }
    }
}

fn select_hex(
    rapier_context: Res<RapierContext>,
    camera_query: Query<(&GlobalTransform, &Camera), With<CurrentCameraTag>>,
    mut hexagon_query: Query<(Entity, &Hexagon, &mut Energy)>,
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
        if let Some((entity, hex, mut energy)) = hexagon_query
            .iter_mut()
            .find(|(ent, _hex, _energy)| ent.id() == entity.id())
        {
            let coord = Offset::from(hex.0);
            info!("{}", coord);

            energy.0 += 1.0;

            // let mut entitycommands = commands.entity(entity);
        }
    }
}

// fn reduce_energy( mut energy_query: Query<&mut Energy>) {
//     for mut energy in energy_query.iter_mut() {
//         energy.0 -= 0.9 * energy.0 * time.delta_seconds();

//         energy.0 = f32::max(0., energy.0);
//     }
// }

fn nan_max(f: &f32, s: &f32) -> Ordering {
    let f = if f.is_nan() { 0. } else { *f };
    let s = if s.is_nan() { 0. } else { *s };

    if f < s {
        Ordering::Less
    } else if s < f {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn distribute_energy(time: Res<Time>, mut query: Query<(&Hexagon, &mut Energy)>) {
    let old_energy: HashMap<Cube, f32> = query
        .iter()
        .map(|(hex, energy)| (hex.0, energy.0))
        .collect();

    let mut new_energy = HashMap::new();

    let dispersal_factor = 0.4;

    for (hex, energy) in query.iter() {
        let dispersed_energy = dispersal_factor * time.delta_seconds() * energy.0;

        let per_neighbour_energy = 1. / 6. * dispersed_energy;

        for neighbour in hex.0.neighbours() {
            *new_energy.entry(neighbour).or_insert(0.) += per_neighbour_energy;
        }
    }
    // info!("{:?}",);

    for (hex, mut energy) in query.iter_mut() {
        energy.0 = energy.0 - 1.1 * dispersal_factor * time.delta_seconds() * energy.0
            + new_energy.get(&hex.0).unwrap_or(&0.);
    }
}
