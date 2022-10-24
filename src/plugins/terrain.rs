use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology, view::NoFrustumCulling},
};
use bevy_rapier3d::prelude::*;
use noise::{NoiseFn, OpenSimplex};

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
    let min = -0.5;
    let max = 0.5;
    let simplex = OpenSimplex::new(rand::random());
    let mut indices = Vec::new();
    let mut vertices = Vec::new();
    let mut last_index = 0;

    for (x, y, z) in (-64..64)
        .flat_map(|x| (0..64).map(move |y| (x, y)))
        .flat_map(|(x, y)| (-64..64).map(move |z| (x as f32, y as f32, z as f32)))
        .filter(|&(x, y, z)| {
            simplex.get([x as f64 / 16.0, y as f64 / 16.0, z as f64 / 16.0]) >= 0.0
        })
    {
        for vertex in [
            // Top
            ([x + min, y + min, z + max], [0., 0., 1.0], [0., 0.]),
            ([x + max, y + min, z + max], [0., 0., 1.0], [1.0, 0.]),
            ([x + max, y + max, z + max], [0., 0., 1.0], [1.0, 1.0]),
            ([x + min, y + max, z + max], [0., 0., 1.0], [0., 1.0]),
            // Bottom
            ([x + min, y + max, z + min], [0., 0., -1.0], [1.0, 0.]),
            ([x + max, y + max, z + min], [0., 0., -1.0], [0., 0.]),
            ([x + max, y + min, z + min], [0., 0., -1.0], [0., 1.0]),
            ([x + min, y + min, z + min], [0., 0., -1.0], [1.0, 1.0]),
            // Right
            ([x + max, y + min, z + min], [1.0, 0., 0.], [0., 0.]),
            ([x + max, y + max, z + min], [1.0, 0., 0.], [1.0, 0.]),
            ([x + max, y + max, z + max], [1.0, 0., 0.], [1.0, 1.0]),
            ([x + max, y + min, z + max], [1.0, 0., 0.], [0., 1.0]),
            // Left
            ([x + min, y + min, z + max], [-1.0, 0., 0.], [1.0, 0.]),
            ([x + min, y + max, z + max], [-1.0, 0., 0.], [0., 0.]),
            ([x + min, y + max, z + min], [-1.0, 0., 0.], [0., 1.0]),
            ([x + min, y + min, z + min], [-1.0, 0., 0.], [1.0, 1.0]),
            // Front
            ([x + max, y + max, z + min], [0., 1.0, 0.], [1.0, 0.]),
            ([x + min, y + max, z + min], [0., 1.0, 0.], [0., 0.]),
            ([x + min, y + max, z + max], [0., 1.0, 0.], [0., 1.0]),
            ([x + max, y + max, z + max], [0., 1.0, 0.], [1.0, 1.0]),
            // Back
            ([x + max, y + min, z + max], [0., -1.0, 0.], [0., 0.]),
            ([x + min, y + min, z + max], [0., -1.0, 0.], [1.0, 0.]),
            ([x + min, y + min, z + min], [0., -1.0, 0.], [1.0, 1.0]),
            ([x + max, y + min, z + min], [0., -1.0, 0.], [0., 1.0]),
        ] {
            vertices.push(vertex);
        }

        for index in [
            0, 1, 2, 2, 3, 0, // top
            4, 5, 6, 6, 7, 4, // bottom
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // front
            20, 21, 22, 22, 23, 20, // back
        ] {
            indices.push(index + last_index);
        }
        last_index += 24;
    }
    let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
    let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
    let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

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
