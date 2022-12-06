use cgmath::Vector2;
use cgmath::Zero;
use cgmath::num_traits::Pow;
use cgmath::num_traits::ToPrimitive;
use wgpu::Device;
use wgpu::util::DeviceExt;

use super::material;
use super::material::ColorUniform;
use super::material::Material;
use super::mesh::Mesh;
use super::square;
use super::mesh;

pub struct Sprite {
    //texture: Option<Texture>,
    pub position: Vector2<f32>,
    pub scale: Vector2<f32>,
    pub material: Material,
}

impl Sprite {
    pub fn empty() -> Self {
        let position = Vector2::zero();
        let scale = Vector2::new(1f32, 1f32);
        let material = Material {
            name: String::from("test"),
            color: wgpu::Color::GREEN,
        };
        
        Self {
            position,
            scale,
            material,
        }
    }
}

pub trait DrawSprite<'a> {
    fn draw_sprite(
        &mut self,
        device: &'a Device,
        sprite: &'a Sprite,
        mesh: &'a mut Mesh,
        camera_bind_group: &'a wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawSprite<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_sprite(
        &mut self,
        device: &'b Device,
        sprite: &'b Sprite,
        mesh: &'b mut Mesh,
        camera_bind_group: &'b wgpu::BindGroup,
    ) {
        let mut square_verts = vec![
            square::SQUARE_VERTS[0],
            square::SQUARE_VERTS[1],
            square::SQUARE_VERTS[2],
            square::SQUARE_VERTS[3],
        ];
        
        for i in 0..square_verts.len() {
            let x = (square_verts[i].x * sprite.scale.x) + sprite.position.x;
            let y = (square_verts[i].y * sprite.scale.y) + sprite.position.y;
            {
                let vert = &mut square_verts[i];
                vert.x = x;
                vert.y = y;
            }
        }
        
        let color_array = [
            sprite.material.color.r.to_f32().unwrap().pow(2.2),
            sprite.material.color.g.to_f32().unwrap().pow(2.2),
            sprite.material.color.b.to_f32().unwrap().pow(2.2),
            sprite.material.color.a.to_f32().unwrap().pow(2.2),
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
        
        let verts: &[mesh::MeshVertex] = &[
            mesh::MeshVertex { position: square_verts[0].into() },
            mesh::MeshVertex { position: square_verts[1].into() },
            mesh::MeshVertex { position: square_verts[2].into() },
            mesh::MeshVertex { position: square_verts[3].into() },
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
        
        mesh.vertex_buffer = vertex_buffer;
        mesh.index_buffer = index_buffer;
        mesh.bind_groups = vec![
            color_bind_group,
        ];
        
        
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        self.set_bind_group(0, camera_bind_group, &[]);
        let mut i = 0;
        for bind_group in &mesh.bind_groups {
            i += 1;
            self.set_bind_group(i, bind_group, &[]);
        }
        self.draw_indexed(0..square::SQUARE_INDICES.len() as u32, 0, 0..1);
    }
}
