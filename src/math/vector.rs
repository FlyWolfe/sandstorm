#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f32,
}

impl Vector {
    pub const fn zero() -> Vector {
        Vector {x: 0.0}
    }
}

#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const fn zero() -> Vector2 {
        Vector2 {x: 0.0, y: 0.0}
    }
}

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const fn zero() -> Vector3 {
        Vector3 {x: 0.0, y: 0.0, z: 0.0}
    }
}