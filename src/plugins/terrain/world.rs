use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use noise::{NoiseFn, OpenSimplex};

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

type VoxelID = u64;

pub struct Chunk {
    pub data: Vec<Vec<Vec<Option<VoxelID>>>>,
}

impl Chunk {
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: vec![vec![vec![None; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_WIDTH],
        }
    }

    pub fn generate(&mut self, noise: &OpenSimplex) {
        for (x, y, z) in (0..CHUNK_WIDTH)
            .flat_map(|x| (0..CHUNK_HEIGHT / 2).map(move |y| (x, y)))
            .flat_map(|(x, y)| (0..CHUNK_WIDTH).map(move |z| (x as f32, y as f32, z as f32)))
            .filter(|&(x, y, z)| {
                noise.get([
                    f64::from(x) / 16.0,
                    f64::from(y) / 16.0,
                    f64::from(z) / 16.0,
                ]) >= 0.0
            })
        {
            self.data[x as usize][y as usize][z as usize] = Some(1);
        }
    }

    pub fn create_mesh(&self) -> Mesh {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();
        let mut last_index = 0;

        for pos in (0..CHUNK_WIDTH)
            .flat_map(|x| (0..CHUNK_HEIGHT).map(move |y| (x, y)))
            .flat_map(|(x, y)| (0..CHUNK_WIDTH).map(move |z| (x, y, z)))
        {
            if self.data[pos.0][pos.1][pos.2].is_some() {
                for (_, p, n, u) in vec![
                    // Front
                    (
                        (0i32, 0i32, 1i32),
                        [
                            [pos.0 as f32 - 0.5, pos.1 as f32 - 0.5, pos.2 as f32 + 0.5],
                            [pos.0 as f32 + 0.5, pos.1 as f32 - 0.5, pos.2 as f32 + 0.5],
                            [pos.0 as f32 + 0.5, pos.1 as f32 + 0.5, pos.2 as f32 + 0.5],
                            [pos.0 as f32 - 0.5, pos.1 as f32 + 0.5, pos.2 as f32 + 0.5],
                        ],
                        [
                            [0.0, 0.0, 1.0],
                            [0.0, 0.0, 1.0],
                            [0.0, 0.0, 1.0],
                            [0.0, 0.0, 1.0],
                        ],
                        [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
                    ),
                    // Back
                    (
                        (0i32, 0i32, -1i32),
                        [
                            [pos.0 as f32 - 0.5, pos.1 as f32 + 0.5, pos.2 as f32 - 0.5],
                            [pos.0 as f32 + 0.5, pos.1 as f32 + 0.5, pos.2 as f32 - 0.5],
                            [pos.0 as f32 + 0.5, pos.1 as f32 - 0.5, pos.2 as f32 - 0.5],
                            [pos.0 as f32 - 0.5, pos.1 as f32 - 0.5, pos.2 as f32 - 0.5],
                        ],
                        [
                            [0.0, 0.0, -1.0],
                            [0.0, 0.0, -1.0],
                            [0.0, 0.0, -1.0],
                            [0.0, 0.0, -1.0],
                        ],
                        [[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
                    ),
                    // Right
                    (
                        (1i32, 0i32, 0i32),
                        [
                            [pos.0 as f32 + 0.5, pos.1 as f32 - 0.5, pos.2 as f32 - 0.5],
                            [pos.0 as f32 + 0.5, pos.1 as f32 + 0.5, pos.2 as f32 - 0.5],
                            [pos.0 as f32 + 0.5, pos.1 as f32 + 0.5, pos.2 as f32 + 0.5],
                            [pos.0 as f32 + 0.5, pos.1 as f32 - 0.5, pos.2 as f32 + 0.5],
                        ],
                        [
                            [1.0, 0.0, 0.0],
                            [1.0, 0.0, 0.0],
                            [1.0, 0.0, 0.0],
                            [1.0, 0.0, 0.0],
                        ],
                        [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
                    ),
                    // Left
                    (
                        (-1i32, 0i32, 0i32),
                        [
                            [pos.0 as f32 - 0.5, pos.1 as f32 - 0.5, pos.2 as f32 + 0.5],
                            [pos.0 as f32 - 0.5, pos.1 as f32 + 0.5, pos.2 as f32 + 0.5],
                            [pos.0 as f32 - 0.5, pos.1 as f32 + 0.5, pos.2 as f32 - 0.5],
                            [pos.0 as f32 - 0.5, pos.1 as f32 - 0.5, pos.2 as f32 - 0.5],
                        ],
                        [
                            [-1.0, 0.0, 0.0],
                            [-1.0, 0.0, 0.0],
                            [-1.0, 0.0, 0.0],
                            [-1.0, 0.0, 0.0],
                        ],
                        [[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
                    ),
                    // Top
                    (
                        (0i32, 1i32, 0i32),
                        [
                            [pos.0 as f32 + 0.5, pos.1 as f32 + 0.5, pos.2 as f32 - 0.5],
                            [pos.0 as f32 - 0.5, pos.1 as f32 + 0.5, pos.2 as f32 - 0.5],
                            [pos.0 as f32 - 0.5, pos.1 as f32 + 0.5, pos.2 as f32 + 0.5],
                            [pos.0 as f32 + 0.5, pos.1 as f32 + 0.5, pos.2 as f32 + 0.5],
                        ],
                        [
                            [0.0, 1.0, 0.0],
                            [0.0, 1.0, 0.0],
                            [0.0, 1.0, 0.0],
                            [0.0, 1.0, 0.0],
                        ],
                        [[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
                    ),
                    // Bottom
                    (
                        (0i32, -1i32, 0i32),
                        [
                            [pos.0 as f32 + 0.5, pos.1 as f32 - 0.5, pos.2 as f32 + 0.5],
                            [pos.0 as f32 - 0.5, pos.1 as f32 - 0.5, pos.2 as f32 + 0.5],
                            [pos.0 as f32 - 0.5, pos.1 as f32 - 0.5, pos.2 as f32 - 0.5],
                            [pos.0 as f32 + 0.5, pos.1 as f32 - 0.5, pos.2 as f32 - 0.5],
                        ],
                        [
                            [0.0, -1.0, 0.0],
                            [0.0, -1.0, 0.0],
                            [0.0, -1.0, 0.0],
                            [0.0, -1.0, 0.0],
                        ],
                        [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.032]],
                    ),
                ]
                .iter()
                .filter(|(face, _, _, _)| {
                    if face.0.wrapping_abs() as u32 as usize > pos.0
                        || face.1.wrapping_abs() as u32 as usize > pos.1
                        || face.2.wrapping_abs() as u32 as usize > pos.2
                    {
                        true
                    } else {
                        fn add(u: usize, i: i32) -> usize {
                            if i.is_negative() {
                                u - i.wrapping_abs() as u32 as usize
                            } else {
                                u + i as usize
                            }
                        }

                        let x: usize = add(pos.0, face.0);
                        let y: usize = add(pos.1, face.1);
                        let z: usize = add(pos.2, face.2);

                        matches!(
                            self.data
                                .get(x)
                                .and_then(|v| v.get(y))
                                .and_then(|v| v.get(z)),
                            None | Some(None)
                        )
                    }
                }) {
                    positions.extend_from_slice(p);
                    normals.extend_from_slice(n);
                    uvs.extend_from_slice(u);
                    indices.extend_from_slice(&[
                        last_index,
                        last_index + 1,
                        last_index + 2,
                        last_index + 2,
                        last_index + 3,
                        last_index,
                    ]);
                    last_index += 4;
                }
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
