use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology, view::NoFrustumCulling},
};
use bevy_rapier3d::prelude::*;

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
) {
    let mut world = world::World::new();
    world.generate();

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    let mut last_index = 0;

    for pos in (0..world::CHUNK_WIDTH)
        .flat_map(|x| (0..world::CHUNK_HEIGHT).map(move |y| (x, y)))
        .flat_map(|(x, y)| (0..world::CHUNK_WIDTH).map(move |z| (x, y, z)))
    {
        let data = world.get_voxel_data_for(pos, last_index);
        let vert_len: u32 = data.0.len().try_into().unwrap();
        last_index += vert_len;
        positions.extend_from_slice(&data.0);
        normals.extend_from_slice(&data.1);
        uvs.extend_from_slice(&data.2);
        indices.extend_from_slice(&data.3);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));

    commands
        .spawn()
        .insert(Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap())
        .insert_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::DARK_GREEN.into()),
            ..default()
        })
        .insert(NoFrustumCulling);
}
