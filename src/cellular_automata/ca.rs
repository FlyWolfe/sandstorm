use cgmath::Vector2;
use wgpu::Color;

pub enum ParticleType {
    Sand,
    Water,
    Fire,
}

pub struct Cell {
    particle_type: ParticleType,
    color: Color,
}

impl Cell {
    pub fn new(particle_type: ParticleType, color: Color) -> Self {
        Self {
            particle_type,
            color,
        }
    }
}

pub struct Chunk {
    cells: Vec<Cell>,
    position: Vector2<f32>,
    width: i32,
    height: i32,
}

impl Chunk {
    pub fn new(cells: Vec<Cell>, position: Vector2<f32>, width: i32, height: i32) -> Self {
        Self {
            cells,
            position,
            width,
            height,
        }
    }
}

pub struct World {
    chunks: Vec<Chunk>,
    width: i32,
    height: i32,
}

impl World {
    pub fn new(chunks: Vec<Chunk>, width: i32, height: i32) -> Self {
        Self {
            chunks,
            width,
            height,
        }
    }
}
