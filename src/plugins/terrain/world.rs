use noise::{NoiseFn, OpenSimplex};

pub const CHUNK_WIDTH: u32 = 32;
pub const CHUNK_HEIGHT: u32 = 32;

type VoxelID = u64;

const AIR: VoxelID = 0;

pub struct World {
    pub simplex: OpenSimplex,
    pub data: [[[VoxelID; CHUNK_WIDTH as usize]; CHUNK_HEIGHT as usize]; CHUNK_WIDTH as usize],
}

type VoxelMeshData = (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>);

impl World {
    #[must_use]
    pub fn new() -> Self {
        Self {
            simplex: OpenSimplex::new(rand::random()),
            data: [[[0; CHUNK_WIDTH as usize]; CHUNK_HEIGHT as usize]; CHUNK_WIDTH as usize],
        }
    }

    pub fn generate(&mut self) {
        for (x, y, z) in (0..CHUNK_WIDTH)
            .flat_map(|x| (0..CHUNK_HEIGHT).map(move |y| (x, y)))
            .flat_map(|(x, y)| (0..CHUNK_WIDTH).map(move |z| (x as f32, y as f32, z as f32)))
            .filter(|&(x, y, z)| {
                self.simplex.get([
                    f64::from(x) / 16.0,
                    f64::from(y) / 16.0,
                    f64::from(z) / 16.0,
                ]) >= 0.0
            })
        {
            self.data[x as usize][y as usize][z as usize] = 1;
        }
    }

    pub fn get_voxel_data_for(&self, pos: (u32, u32, u32), last_index: u32) -> VoxelMeshData {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        for (_, p, n, u, i) in [
            // Top
            (
                (0i32, 1i32, 0i32),
                [
                    [pos.0 as f32 + -0.5, pos.1 as f32 + -0.5, pos.2 as f32 + 0.5],
                    [pos.0 as f32 + 0.5, pos.1 as f32 + -0.5, pos.2 as f32 + 0.5],
                    [pos.0 as f32 + 0.5, pos.1 as f32 + 0.5, pos.2 as f32 + 0.5],
                    [pos.0 as f32 + -0.5, pos.1 as f32 + 0.5, pos.2 as f32 + 0.5],
                ],
                [
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0, 1.0],
                ],
                [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
                [
                    last_index,
                    last_index + 1,
                    last_index + 2,
                    last_index + 2,
                    last_index + 3,
                    last_index,
                ],
            ),
            // Bottom
            (
                (0i32, -1i32, 0i32),
                [
                    [pos.0 as f32 + -0.5, pos.1 as f32 + 0.5, pos.2 as f32 + -0.5],
                    [pos.0 as f32 + 0.5, pos.1 as f32 + 0.5, pos.2 as f32 + -0.5],
                    [pos.0 as f32 + 0.5, pos.1 as f32 + -0.5, pos.2 as f32 + -0.5],
                    [
                        pos.0 as f32 + -0.5,
                        pos.1 as f32 + -0.5,
                        pos.2 as f32 + -0.5,
                    ],
                ],
                [
                    [0.0, 0.0, -1.0],
                    [0.0, 0.0, -1.0],
                    [0.0, 0.0, -1.0],
                    [0.0, 0.0, -1.0],
                ],
                [[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
                [
                    last_index + 4,
                    last_index + 5,
                    last_index + 6,
                    last_index + 6,
                    last_index + 7,
                    last_index + 4,
                ],
            ),
            // Right
            (
                (1i32, 0i32, 0i32),
                [
                    [pos.0 as f32 + 0.5, pos.1 as f32 + -0.5, pos.2 as f32 + -0.5],
                    [pos.0 as f32 + 0.5, pos.1 as f32 + 0.5, pos.2 as f32 + -0.5],
                    [pos.0 as f32 + 0.5, pos.1 as f32 + 0.5, pos.2 as f32 + 0.5],
                    [pos.0 as f32 + 0.5, pos.1 as f32 + -0.5, pos.2 as f32 + 0.5],
                ],
                [
                    [1.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                ],
                [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
                [
                    last_index + 8,
                    last_index + 9,
                    last_index + 10,
                    last_index + 10,
                    last_index + 11,
                    last_index + 8,
                ],
            ),
            // Left
            (
                (-1i32, 0i32, 0i32),
                [
                    [pos.0 as f32 + -0.5, pos.1 as f32 + -0.5, pos.2 as f32 + 0.5],
                    [pos.0 as f32 + -0.5, pos.1 as f32 + 0.5, pos.2 as f32 + 0.5],
                    [pos.0 as f32 + -0.5, pos.1 as f32 + 0.5, pos.2 as f32 + -0.5],
                    [
                        pos.0 as f32 + -0.5,
                        pos.1 as f32 + -0.5,
                        pos.2 as f32 + -0.5,
                    ],
                ],
                [
                    [-1.0, 0.0, 0.0],
                    [-1.0, 0.0, 0.0],
                    [-1.0, 0.0, 0.0],
                    [-1.0, 0.0, 0.0],
                ],
                [[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
                [
                    last_index + 12,
                    last_index + 13,
                    last_index + 14,
                    last_index + 14,
                    last_index + 15,
                    last_index + 12,
                ],
            ),
            // Front
            (
                (0i32, 0i32, 1i32),
                [
                    [pos.0 as f32 + 0.5, pos.1 as f32 + 0.5, pos.2 as f32 + -0.5],
                    [pos.0 as f32 + -0.5, pos.1 as f32 + 0.5, pos.2 as f32 + -0.5],
                    [pos.0 as f32 + -0.5, pos.1 as f32 + 0.5, pos.2 as f32 + 0.5],
                    [pos.0 as f32 + 0.5, pos.1 as f32 + 0.5, pos.2 as f32 + 0.5],
                ],
                [
                    [0.0, 1.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 1.0, 0.0],
                ],
                [[1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
                [
                    last_index + 16,
                    last_index + 17,
                    last_index + 18,
                    last_index + 18,
                    last_index + 19,
                    last_index + 16,
                ],
            ),
            // Back
            (
                (0i32, 0i32, -1i32),
                [
                    [pos.0 as f32 + 0.5, pos.1 as f32 + -0.5, pos.2 as f32 + 0.5],
                    [pos.0 as f32 + -0.5, pos.1 as f32 + -0.5, pos.2 as f32 + 0.5],
                    [
                        pos.0 as f32 + -0.5,
                        pos.1 as f32 + -0.5,
                        pos.2 as f32 + -0.5,
                    ],
                    [pos.0 as f32 + 0.5, pos.1 as f32 + -0.5, pos.2 as f32 + -0.5],
                ],
                [
                    [0.0, -1.0, 0.0],
                    [0.0, -1.0, 0.0],
                    [0.0, -1.0, 0.0],
                    [0.0, -1.0, 0.0],
                ],
                [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.032]],
                [
                    last_index + 20,
                    last_index + 21,
                    last_index + 22,
                    last_index + 22,
                    last_index + 23,
                    last_index + 20,
                ],
            ),
        ]
        .iter()
        .filter(|(face, _, _, _, _)| {
            if face.0.wrapping_abs() as u32 > pos.0
                || face.1.wrapping_abs() as u32 > pos.1
                || face.2.wrapping_abs() as u32 > pos.2
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

                let x: usize = add(pos.0 as usize, face.0);
                let y: usize = add(pos.1 as usize, face.1);
                let z: usize = add(pos.2 as usize, face.2);

                if x >= CHUNK_WIDTH as usize
                    || y >= CHUNK_HEIGHT as usize
                    || z >= CHUNK_WIDTH as usize
                {
                    true
                } else {
                    matches!(
                        self.data
                            .get(x)
                            .and_then(|v| v.get(y))
                            .and_then(|v| v.get(z)),
                        None | Some(&AIR)
                    )
                }
            }
        }) {
            positions.extend_from_slice(p);
            normals.extend_from_slice(n);
            uvs.extend_from_slice(u);
            indices.extend_from_slice(i);
        }

        (positions, normals, uvs, indices)
    }
}
