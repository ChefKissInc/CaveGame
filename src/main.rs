#![warn(clippy::cargo, unused_extern_crates)]

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::ProgressPlugin;

mod plugins;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AppState {
    Splash,
    InGame,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Cave Game (Working Title)".to_owned(),
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_loopless_state(AppState::Splash)
        .add_plugin(
            ProgressPlugin::new(AppState::Splash)
                .continue_to(AppState::InGame)
                .track_assets(),
        )
        .add_enter_system(AppState::InGame, setup)
        .add_plugin(plugins::resources::GameResourcePlugin)
        .add_plugin(plugins::terrain::TerrainPlugin)
        .add_plugin(plugins::hud::HudPlugin)
        .add_plugin(plugins::player::PlayerPlugin)
        .add_system(cursor_lock_manager.run_in_state(crate::AppState::InGame))
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_grab_mode(bevy::window::CursorGrabMode::Locked);
    window.set_cursor_visibility(false);

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 5.0, 0.0),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                -std::f32::consts::PI / 4.0,
                std::f32::consts::PI / 4.0,
                0.0,
            ),
            ..default()
        },
        ..default()
    });
}

fn cursor_lock_manager(
    mut windows: ResMut<Windows>,
    input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    let window = windows.get_primary_mut().unwrap();
    if input.just_pressed(KeyCode::Escape) {
        window.set_cursor_grab_mode(bevy::window::CursorGrabMode::None);
        window.set_cursor_visibility(true);
    }
    if mouse_button_input.just_pressed(MouseButton::Left) {
        window.set_cursor_grab_mode(bevy::window::CursorGrabMode::Locked);
        window.set_cursor_visibility(false);
    }
}
