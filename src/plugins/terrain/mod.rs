use bevy::{prelude::*, render::view::NoFrustumCulling};
use bevy_rapier3d::prelude::*;
use noise::OpenSimplex;

pub mod world;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(terrain_setup);
    }
}

fn terrain_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let simplex = OpenSimplex::new(rand::random());
    let texture = asset_server.load("textures/grass_block_top.png");

    for x in 0..10 {
        for z in 0..10 {
            let mut chunk = world::Chunk::new();
            chunk.generate(&simplex, (x * world::CHUNK_WIDTH, z * world::CHUNK_WIDTH));
            let mesh = chunk.create_mesh();

            commands
                .spawn()
                .insert(Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap())
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(StandardMaterial {
                        base_color: Color::GREEN,
                        base_color_texture: Some(texture.clone()),
                        alpha_mode: AlphaMode::Opaque,
                        metallic: 0.0,
                        perceptual_roughness: 0.7,
                        ..default()
                    }),
                    transform: Transform::from_translation(Vec3::new(
                        -(5.0 * world::CHUNK_WIDTH as f32) + x as f32 * world::CHUNK_WIDTH as f32,
                        0.0,
                        -(5.0 * world::CHUNK_WIDTH as f32) + z as f32 * world::CHUNK_WIDTH as f32,
                    )),
                    ..default()
                })
                .insert(NoFrustumCulling);
        }
    }
}
