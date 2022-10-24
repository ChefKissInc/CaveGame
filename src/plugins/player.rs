use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_atmosphere::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_setup)
            .add_system(control_system);
    }
}

fn player_setup(mut commands: Commands) {
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(0.0, 70.0, 0.0)))
        .insert(AtmosphereCamera(None))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(ReadMassProperties::default())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::cuboid(0.5, 1.0, 0.5))
        .insert(Ccd::enabled())
        .insert(Sleeping::disabled())
        .insert(CameraController::default())
        .with_children(|v| {
            v.spawn_bundle(Camera3dBundle::default());
        });
}

#[derive(Clone, Component, Copy, Debug)]
struct CameraController {
    pub mouse_rotate_sensitivity: Vec2,
    pub yaw_pitch_roll: Vec2,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            mouse_rotate_sensitivity: Vec2::splat(0.2),
            yaw_pitch_roll: Vec2::ZERO,
        }
    }
}

const PITCH_BOUND: f32 = std::f32::consts::FRAC_PI_2 - 1E-3;

fn control_system(
    keyboard: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut controllers: Query<(
        &mut CameraController,
        &ReadMassProperties,
        &Velocity,
        &mut ExternalForce,
        &mut ExternalImpulse,
        &mut Transform,
    )>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<CameraController>)>,
    time: Res<Time>,
) {
    let (mut controller, mass, vel, mut force, mut impulse, mut head_transform) =
        controllers.single_mut();
    let mut transform = cameras.single_mut();
    let dt = time.delta_seconds();

    let mut cursor_delta = Vec2::ZERO;
    for event in mouse_motion_events.iter() {
        cursor_delta -= event.delta;
    }

    if cursor_delta.length_squared() > 1E-6 {
        cursor_delta = dt * (cursor_delta * controller.mouse_rotate_sensitivity);
        controller.yaw_pitch_roll += cursor_delta;
        controller.yaw_pitch_roll.y = controller.yaw_pitch_roll.y.clamp(-PITCH_BOUND, PITCH_BOUND);
        // Yaw
        head_transform.rotation = Quat::from_rotation_y(controller.yaw_pitch_roll.x);
        // Pitch
        transform.rotation = Quat::from_rotation_x(controller.yaw_pitch_roll.y);
    }

    let xz = Vec3::new(1.0, 0.0, 1.0);
    let rotation = Quat::from_rotation_x(controller.yaw_pitch_roll.y)
        * Quat::from_rotation_y(controller.yaw_pitch_roll.x);
    let right = ((rotation * Vec3::X) * xz).normalize();
    let forward = ((rotation * -Vec3::Z) * xz).normalize();
    let mut desired_velocity = Vec3::ZERO;

    for dir in [
        (KeyCode::W, forward),
        (KeyCode::S, -forward),
        (KeyCode::D, right),
        (KeyCode::A, -right),
    ]
    .iter()
    .filter(|(key, _)| keyboard.pressed(*key))
    .map(|(_, dir)| dir)
    {
        desired_velocity += *dir;
    }

    desired_velocity = if desired_velocity.length_squared() > 1E-6 {
        desired_velocity.normalize() * 5.0
    } else {
        vel.linvel * 0.5 * xz
    };

    // Calculate impulse - the desired momentum change for the time period
    let delta_velocity = desired_velocity - vel.linvel * xz;
    let impulse_val = delta_velocity * mass.0.mass;
    if impulse_val.length_squared() > 1E-6 {
        impulse.impulse = impulse_val;
    }

    // Calculate force - the desired rate of change of momentum for the time period
    let force_val = impulse_val / dt;
    if force_val.length_squared() > 1E-6 {
        force.force = force_val;
    }
}
