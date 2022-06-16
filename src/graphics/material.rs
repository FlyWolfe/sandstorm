
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorUniform {
    pub value: [f32; 4],
}

pub struct Material {
    pub name: String,
    pub color: wgpu::Color,
}