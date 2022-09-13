use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    coordinates::Axial, coordinates::Cube, coordinates::Offset,
    input::camera_control_plugin::CurrentCameraTag, mesh_generation::hex::create_hex_prism,
};

use itertools::Itertools;

use bevy_trafo::Trafo;

#[derive(Component)]
struct Hexagon(Cube);

#[derive(Component)]
struct Energy {
    velocity: Vec3,
    // acceleration: Vec3,
}

pub struct HexWorld;

impl Plugin for HexWorld {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(select_hex)
            .add_system(integrate)
            .add_system(distribute_velocity.after(integrate));
    }
}

fn get_coordinates() -> impl Iterator<Item = Axial> {
    (-25..=25)
        .cartesian_product(-25..=25)
        .map(|(x, y)| Offset::new(x, y).into())

    // Cube::origin()
    //     .neighbours()
    //     .map(|c| c.into())
    //     .chain(std::iter::once(Axial::origin()))
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

    for axial in get_coordinates() {
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
            .insert(Energy {
                velocity: Vec3::ZERO,
            });
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

            energy.velocity += Vec3::Y * 30.;
        }
    }
}

fn integrate(time: Res<Time>, mut query: Query<(&mut Transform, &mut Energy)>) {
    for (mut t, mut e) in query.iter_mut() {
        let spring = 10.;
        let mass = 1.0;
        let x = t.translation.y;
        let damp = 1.5;
        let acceleration = -(spring / mass) * x - damp * e.velocity.y;

        let acceleration = acceleration * Vec3::Y;

        t.translation += time.delta_seconds() * e.velocity;
        e.velocity += time.delta_seconds() * acceleration;

        if e.velocity.y.abs() <= 0.0001 {
            e.velocity.y = 0.
        }
    }
}

fn distribute_velocity(time: Res<Time>, mut query: Query<(&Hexagon, &Transform, &mut Energy)>) {
    let height_map: HashMap<_, _> = query
        .iter()
        .map(|(h, t, _)| (h.0, t.translation.y))
        .collect();

    // info!("{:?}", height_map);

    let mut dist = HashMap::new();

    for (hex, t, mut e) in query.iter_mut() {
        // if e.velocity.y.abs() < f32::EPSILON {
        //     continue;
        // }
        for neighbour in hex.0.neighbours() {
            let spring = 1.;
            let mass = 1.0;
            let x = (t.translation.y - height_map.get(&neighbour).unwrap_or(&0.0)).abs();
            let damp = 0.1;
            let acceleration = -(spring / mass) * x - damp * e.velocity.y;

            // info!("{} {} {}", hex.0, neighbour, acceleration);
            let acceleration = 0.9 * acceleration * Vec3::Y;
            *dist.entry(neighbour).or_insert(Vec3::ZERO) -= acceleration;
            *dist.entry(hex.0).or_insert(Vec3::ZERO) += acceleration;
        }
    }

    // info!("END");
    for (hex, t, mut e) in query.iter_mut() {
        e.velocity += *dist.get(&hex.0).unwrap_or(&Vec3::ZERO);
    }
}
