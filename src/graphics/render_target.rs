use std::sync::Arc;

use wgpu::TextureView;

use super::{context::{GraphicsContext, WgpuContext}, mesh::{Mesh, DrawMesh}};

pub struct RenderTarget {
    pub(crate) wgpu: Arc<WgpuContext>,
    pub(crate) camera_bind_group: wgpu::BindGroup,
    pub mesh: Mesh,
}

impl RenderTarget {
    pub fn finalize(&mut self, gfx: &mut GraphicsContext, encoder: &mut wgpu::CommandEncoder, view: &mut TextureView) -> Result<(), anyhow::Error> {
        let mut renderTarget = RawRenderTarget::new(gfx, |encoder| {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            })
        })?;
        
        renderTarget.pass.draw_mesh(&self.mesh, &self.camera_bind_group);
        
        Ok(())
    }
}

pub struct RawRenderTarget<'a> {
    wgpu: &'a WgpuContext,
    pass: wgpu::RenderPass<'a>,
}

impl<'a> RawRenderTarget<'a> {
    pub(crate) fn new(
        gfx: &'a mut GraphicsContext,
        create_pass: impl FnOnce(&'a mut wgpu::CommandEncoder) -> wgpu::RenderPass<'a>,
    ) -> Result<Self, anyhow::Error> {
        
        if gfx.fcx.is_none() {
            return Err(anyhow::anyhow!("Starting Canvas outside of a frame"));
        }
        
        let wgpu = &gfx.wgpu;
        
        let mut pass = {
            let fcx = gfx.fcx.as_mut().unwrap(/* Error handled above */);
            
            create_pass(&mut fcx.cmd)
        };
        
        pass.set_blend_constant(wgpu::Color::BLACK);
        
        Ok(RawRenderTarget {
            wgpu,
            pass,
        })
    }
}
