use cgmath::Vector3;

pub const SQUARE_VERTS: &[Vector3<f32>] = &[
    Vector3 { x: -0.5, y: 0.5, z: 0.0 },
    Vector3 { x: -0.5, y: -0.5, z: 0.0 },
    Vector3 { x: 0.5, y: -0.5, z: 0.0 },
    Vector3 { x: 0.5, y: 0.5, z: 0.0 },
];

pub const SQUARE_INDICES: &[u16] = &[
    0, 1, 2,
    2, 3, 0,
];