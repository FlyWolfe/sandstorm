use cgmath::Vector2;
use cgmath::Vector3;
use cgmath::num_traits::Pow;
use cgmath::num_traits::ToPrimitive;
use wgpu::Buffer;
use wgpu::BufferBinding;
use wgpu::Color;
use wgpu::util::DeviceExt;

use super::model::DrawModel;
use super::texture::Texture;
use super::square;
use super::model;

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
        
        let verts: &[model::ModelVertex] = &[
            model::ModelVertex { position: square_verts[0].into(), color: color_array },
            model::ModelVertex { position: square_verts[1].into(), color: color_array },
            model::ModelVertex { position: square_verts[2].into(), color: color_array },
            model::ModelVertex { position: square_verts[3].into(), color: color_array },
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
