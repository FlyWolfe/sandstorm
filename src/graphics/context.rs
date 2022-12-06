use std::sync::Arc;



#[derive(Debug)]
pub struct WgpuContext {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

pub struct GraphicsContext {
    pub(crate) wgpu: Arc<WgpuContext>,
    pub(crate) fcx: Option<FrameContext>,
    pub(crate) window: winit::window::Window,
    pub(crate) surface_format: wgpu::TextureFormat,
}

pub struct FrameContext {
    pub cmd: wgpu::CommandEncoder,
    pub render_pipeline: wgpu::RenderPipeline,
}