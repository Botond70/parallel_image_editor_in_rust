use dioxus_native::{CustomPaintCtx, CustomPaintSource, DeviceHandle, TextureHandle};
use dioxus_native::*;
use image::{DynamicImage, GenericImageView};
use wgpu::*;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct PaintSource {
    state: RendererState,
    tx: Sender<Message>,
    rx: Receiver<Message>,
    image: Option<DynamicImage>,
}

impl CustomPaintSource for PaintSource {
    fn resume(&mut self, _instance: &Instance, device_handle: &DeviceHandle) {
        let device = &device_handle.device;
        let queue = &device_handle.queue;
        let active_state = ActiveRenderer::new(device, queue);
        self.state = RendererState::Active(Box::new(active_state));
    }

    fn suspend(&mut self) {
        self.state = RendererState::Suspended;
    }

    fn render(
        &mut self,
        ctx: CustomPaintCtx<'_>,
        width: u32,
        height: u32,
        _scale: f64,
    ) -> Option<TextureHandle> {
        self.process_messages();
        self.render(ctx, width, height)
    }
}

pub enum Message {
    SetImage(DynamicImage),
}

enum RendererState {
    Active(Box<ActiveRenderer>),
    Suspended,
}

#[derive(Clone)]
struct TextureAndHandle {
    texture: Texture,
    handle: TextureHandle,
}

struct ActiveRenderer {
    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,
    displayed_texture: Option<TextureAndHandle>,
}

impl PaintSource {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            state: RendererState::Suspended,
            tx,
            rx,
            image: None,
        }
    }

    pub fn sender(&self) -> Sender<Message> {
        self.tx.clone()
    }

    fn process_messages(&mut self) {
        loop {
            match self.rx.try_recv() {
                Err(_) => return,
                Ok(msg) => match msg {
                    Message::SetImage(image) => self.image = Some(image),
                }
            }
        }
    }

    fn render(
        &mut self,
        ctx: CustomPaintCtx<'_>,
        width: u32,
        height: u32,
    ) -> Option<TextureHandle> {
        if width == 0 || height == 0 {
            return None;
        }
        let RendererState::Active(state) = &mut self.state else {
            return None;
        };

        state.render(ctx, width, height, self.image.as_ref().unwrap())
    }
}

impl ActiveRenderer {
    pub(crate) fn new(device: &Device, queue: &Queue) -> Self {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[PushConstantRange {
                stages: ShaderStages::FRAGMENT,
                range: 0..16,
            }],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(TextureFormat::Rgba8Unorm.into())],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            device: device.clone(),
            queue: queue.clone(),
            pipeline,
            displayed_texture: None,
        }
    }

    pub(crate) fn render(&mut self, mut ctx: CustomPaintCtx<'_>, width: u32, height: u32, image: &DynamicImage) -> Option<TextureHandle> {
        let texture_and_handle = match &self.displayed_texture {
            Some(texture) => texture,
            None => {
                let texture = create_texture(&self.device, &self.queue, width, height, image);
                let handle = ctx.register_texture(texture.clone());
                self.displayed_texture = Some(TextureAndHandle { texture: texture, handle: handle});
                self.displayed_texture.as_ref().unwrap()
            }
        };

        let texture = &texture_and_handle.texture;
        let handle = texture_and_handle.handle;
        let texture_view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = self.device.create_sampler(&SamplerDescriptor {
            label: Some("image_sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group_layout = self.device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("bind_group_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = self.device.create_bind_group(&BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("bind_group"),
        });

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor { label: Some("render_encoder") });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);
            render_pass.draw(0..6, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));

        Some(handle)
    }
}

fn create_texture(device: &Device, queue: &Queue, width: u32, height: u32, image: &DynamicImage) -> Texture {
    let texture = device.create_texture(&TextureDescriptor {
        label: None,
        size: Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    let rgba = image.to_rgba8();
    let image_dimensions = image.dimensions();
    let texture_size = wgpu::Extent3d {
        width: image_dimensions.0,
        height: image_dimensions.1,
        depth_or_array_layers: 1,
    };

    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &rgba,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: std::num::NonZeroU32::new(4 * image_dimensions.0).map(|n| n.get()),
            rows_per_image: std::num::NonZeroU32::new(image_dimensions.1).map(|n| n.get()),
        },
        texture_size,
    );

    texture
}