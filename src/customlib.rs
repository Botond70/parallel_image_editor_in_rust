use web_sys::*;
use wgpu::*;
pub struct State {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub is_surface_configured: bool,
    // NEW!
    pub render_pipeline: wgpu::RenderPipeline,
}
