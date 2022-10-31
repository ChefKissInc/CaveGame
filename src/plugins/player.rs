use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

#[derive(Actionlike, Clone, Copy)]
enum PlayerInputMap {
    PanCamera,
    MoveForward,
    MoveBackwards,
    MoveLeft,
    MoveRight,
    Run,
    Jump,
    Crouch,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AtmospherePlugin)
            .insert_resource(AtmosphereSettings { resolution: 1024 })
            .add_plugin(InputManagerPlugin::<PlayerInputMap>::default())
            .add_startup_system(player_setup)
            .add_system(control_system);
    }
}

fn player_setup(mut commands: Commands) {
    let mut input_map = InputMap::new([
        (KeyCode::W, PlayerInputMap::MoveForward),
        (KeyCode::S, PlayerInputMap::MoveBackwards),
        (KeyCode::A, PlayerInputMap::MoveLeft),
        (KeyCode::D, PlayerInputMap::MoveRight),
        (KeyCode::LShift, PlayerInputMap::Run),
        (KeyCode::RShift, PlayerInputMap::Run),
        (KeyCode::Space, PlayerInputMap::Jump),
        (KeyCode::LControl, PlayerInputMap::Crouch),
        (KeyCode::RControl, PlayerInputMap::Crouch),
    ]);
    input_map.insert(DualAxis::mouse_motion(), PlayerInputMap::PanCamera);
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(0.0, 138.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(ExternalForce::default())
        .insert(ExternalImpulse::default())
        .insert(ReadMassProperties::default())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::cuboid(0.5, 1.0, 0.5))
        .insert(Ccd::enabled())
        .insert(Sleeping::disabled())
        .insert(PlayerController::default())
        .insert_bundle(InputManagerBundle::<PlayerInputMap> {
            action_state: ActionState::default(),
            input_map,
        })
        .insert_bundle(VisibilityBundle::default())
        .with_children(|v| {
            v.spawn_bundle(Camera3dBundle {
                transform: Transform::from_xyz(0.0, 1.0, 0.0),
                ..default()
            })
            .insert(AtmosphereCamera(None));
        });
}

#[derive(Clone, Component, Copy, Debug)]
struct PlayerController {
    pub mouse_rotate_sensitivity: Vec2,
    pub yaw_pitch: Vec2,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            mouse_rotate_sensitivity: Vec2::splat(0.01),
            yaw_pitch: Vec2::ZERO,
        }
    }
}

const PITCH_BOUND: f32 = std::f32::consts::FRAC_PI_2 - 1E-3;
const LAG_WEIGHT: f32 = 0.8;

type PlayerQuery<'a> = (
    &'a ActionState<PlayerInputMap>,
    &'a mut PlayerController,
    &'a ReadMassProperties,
    &'a Velocity,
    &'a mut ExternalForce,
    &'a mut ExternalImpulse,
    &'a mut Transform,
);

fn control_system(
    mut controllers: Query<PlayerQuery>,
    mut cameras: Query<&mut Transform, (With<Camera3d>, Without<PlayerController>)>,
    time: Res<Time>,
) {
    let (action, mut controller, mass, vel, mut force, mut impulse, mut transform) =
        controllers.single_mut();
    let mut head_transform = cameras.single_mut();
    let dt = time.delta_seconds();

    let mut cursor_delta = -action.axis_pair(PlayerInputMap::PanCamera).unwrap().xy();
    cursor_delta *= controller.mouse_rotate_sensitivity;

    let old = controller.yaw_pitch;
    controller.yaw_pitch += cursor_delta;
    controller.yaw_pitch.y = controller.yaw_pitch.y.clamp(-PITCH_BOUND, PITCH_BOUND);
    controller.yaw_pitch = old * LAG_WEIGHT + controller.yaw_pitch * (1.0 - LAG_WEIGHT);
    // Yaw
    transform.rotation =
        Quat::from_rotation_y(controller.yaw_pitch.x).lerp(transform.rotation, LAG_WEIGHT);
    // Pitch
    head_transform.rotation =
        Quat::from_rotation_x(controller.yaw_pitch.y).lerp(head_transform.rotation, LAG_WEIGHT);

    let xz = Vec3::new(1.0, 0.0, 1.0);
    let rotation = Quat::from_rotation_x(controller.yaw_pitch.y)
        * Quat::from_rotation_y(controller.yaw_pitch.x);
    let right = ((rotation * Vec3::X) * xz).normalize();
    let forward = ((rotation * -Vec3::Z) * xz).normalize();
    let mut desired_velocity = Vec3::ZERO;

    for dir in [
        (PlayerInputMap::MoveForward, forward),
        (PlayerInputMap::MoveBackwards, -forward),
        (PlayerInputMap::MoveRight, right),
        (PlayerInputMap::MoveLeft, -right),
        (PlayerInputMap::Jump, Vec3::Y),
        (PlayerInputMap::Crouch, -Vec3::Y),
    ]
    .iter()
    .filter(|(key, _)| action.pressed(*key))
    .map(|(_, dir)| dir)
    {
        desired_velocity += *dir;
    }

    let speed = if action.pressed(PlayerInputMap::Run) {
        12.0
    } else {
        8.0
    };
    desired_velocity = if desired_velocity.length_squared() > 1E-6 {
        desired_velocity.normalize() * speed
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
    let force_val = impulse_val * dt;
    if force_val.length_squared() > 1E-6 {
        force.force = force_val;
    }
}
