
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorUniform {
    pub value: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub color: wgpu::Color,
}