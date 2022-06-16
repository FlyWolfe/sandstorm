use cgmath::Vector2;
use cgmath::Vector3;
use cgmath::num_traits::Pow;
use cgmath::num_traits::ToPrimitive;
use wgpu::Color;
use wgpu::util::DeviceExt;

use super::square;
use super::model;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ColorUniform {
    value: [f32; 4],
}

pub struct Sprite {
    //texture: Option<Texture>,
    pub position: Vector2<f32>,
    pub scale: Vector2<f32>,
    pub color: Color,
    pub model: model::Model,
}

impl Sprite {
    pub fn new(position: Vector2<f32>, scale: Vector2<f32>, color: Color, device: &wgpu::Device) -> Self {
        let mut square_verts = vec![
            Vector3 { x: -0.5, y: 0.5, z: 0.0 },
            Vector3 { x: -0.5, y: -0.5, z: 0.0 },
            Vector3 { x: 0.5, y: -0.5, z: 0.0 },
            Vector3 { x: 0.5, y: 0.5, z: 0.0 },
        ];
        
        for i in 0..square_verts.len() {
            let x = (square_verts[i].x * scale.x) + position.x;
            let y = (square_verts[i].y * scale.y) + position.y;
            {
                let vert = &mut square_verts[i];
                vert.x = x;
                vert.y = y;
            }
        }
        
        
        let color_array = [
            color.r.to_f32().unwrap().pow(2.2),
            color.g.to_f32().unwrap().pow(2.2),
            color.b.to_f32().unwrap().pow(2.2),
            color.a.to_f32().unwrap().pow(2.2),
        ];
        
        
        let color_uniform = ColorUniform { value: color_array };
        
        let color_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[color_uniform]),
                usage: wgpu::BufferUsages::UNIFORM,
            }
        );
        
        let color_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("colors_bind_group_layout"),
        });
        
        let color_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &color_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: color_buffer.as_entire_binding(),
                }
            ],
            label: Some("color_bind_group"),
        });
        
        let verts: &[model::ModelVertex] = &[
            model::ModelVertex { position: square_verts[0].into() },
            model::ModelVertex { position: square_verts[1].into() },
            model::ModelVertex { position: square_verts[2].into() },
            model::ModelVertex { position: square_verts[3].into() },
        ];
        
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(verts),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(square::SQUARE_INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        
        let material = model::Material {
            name: "Temp Material".to_string(),
            color: color,
            color_bind_group: color_bind_group,
        };
        
        let model = model::Model {
            name: "Temp Model".to_string(),
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            num_elements: square::SQUARE_INDICES.len() as u32,
            material: material,
        };
        
        Self {
            position,
            scale,
            color,
            model,
        }
    }
}
