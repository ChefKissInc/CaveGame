use bevy::prelude::*;
// use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

mod plugins;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Minecraft 420".to_owned(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        // .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_plugin(plugins::player::PlayerPlugin)
        .add_plugin(plugins::terrain::TerrainPlugin)
        .add_plugin(plugins::hud::HUDPlugin)
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 5.0, 0.0),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                -std::f32::consts::PI / 4.0,
                std::f32::consts::PI / 4.0,
                0.0,
            ),
            ..Default::default()
        },
        ..Default::default()
    });
}
