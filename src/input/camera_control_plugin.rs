use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Component)]
pub struct CurrentCameraTag;

pub struct CameraControlSettings {
    translation_speed: f32,
    translation_speed_fast: f32,
    rotation_speed: f32,
}

impl Default for CameraControlSettings {
    fn default() -> Self {
        Self {
            translation_speed: 5.0,
            translation_speed_fast: 25.0,
            rotation_speed: 3.0,
        }
    }
}

pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        if let None = app.world.get_resource::<CameraControlSettings>() {
            app.insert_resource::<CameraControlSettings>(Default::default());
        }

        app.add_system(move_camera)
            .add_system(rotate_camera)
            .add_system(cursor_grab_system)
            .add_startup_system(create_camera);
    }
}

fn create_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(5.0, 15., 5.)
                .looking_at(Vec3::new(5.0, 0.0, -5.0), Vec3::NEG_Z),
            ..default()
        })
        .insert(CurrentCameraTag);
}

fn move_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    settings: Res<CameraControlSettings>,
    mut query: Query<&mut Transform, With<CurrentCameraTag>>,
) {
    let mut transform = query.single_mut();

    let mut delta_pos = Vec3::ZERO;

    if keys.pressed(KeyCode::Right) || keys.pressed(KeyCode::D) {
        delta_pos += Vec3::X;
    }
    if keys.pressed(KeyCode::Left) || keys.pressed(KeyCode::A) {
        delta_pos += Vec3::NEG_X;
    }
    if keys.pressed(KeyCode::Up) || keys.pressed(KeyCode::W) {
        delta_pos += Vec3::NEG_Z;
    }
    if keys.pressed(KeyCode::Down) || keys.pressed(KeyCode::S) {
        delta_pos += Vec3::Z;
    }
    if keys.pressed(KeyCode::Q) || keys.pressed(KeyCode::PageDown) {
        delta_pos += Vec3::NEG_Y;
    }
    if keys.pressed(KeyCode::E) || keys.pressed(KeyCode::PageUp) {
        delta_pos += Vec3::Y;
    }

    let speed = if keys.pressed(KeyCode::LShift) {
        settings.translation_speed_fast
    } else {
        settings.translation_speed
    };

    let delta_pos = delta_pos.normalize_or_zero();
    let rot = transform.rotation;
    transform.translation += speed * time.delta_seconds() * (rot * delta_pos);
}

fn rotate_camera(
    windows: Res<Windows>,
    time: Res<Time>,
    settings: Res<CameraControlSettings>,
    mut query: Query<&mut Transform, With<CurrentCameraTag>>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    let window = windows.get_primary().unwrap();

    if !window.cursor_locked() {
        return;
    }

    let delta: Vec2 = motion_evr
        .iter()
        .map(|ev| ev.delta)
        .reduce(|a, b| a + b)
        .unwrap_or(Vec2::ZERO);

    let delta = settings.rotation_speed * time.delta_seconds() * delta;

    let quat_up_down = Quat::from_euler(EulerRot::XYZ, -delta.y.to_radians(), 0., 0.);
    let quat_left_right = Quat::from_rotation_y(-delta.x.to_radians());
    let mut transform = query.single_mut();
    transform.rotate_local(quat_up_down);
    transform.rotate(quat_left_right);

    info!("{:?}", transform.rotation.to_euler(EulerRot::XYZ));
}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) || btn.just_pressed(MouseButton::Right) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}
