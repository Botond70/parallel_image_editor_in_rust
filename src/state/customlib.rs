use crate::state::app_state::HSVState;
use crate::utils::utils::{align_to_256, save_file_via_dialog};
use dioxus::hooks::use_context;
use dioxus::html::output;
use dioxus::html::u::is;
use image::DynamicImage;
use image::GenericImageView;
use image::{ImageBuffer, Rgba};
use std::collections::VecDeque;
use std::sync::mpsc::channel;
use std::sync::mpsc::{self, RecvError};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::spawn;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::*;
use wgpu::util::DeviceExt;
use wgpu::*;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Globals {
    pub hsv: [f32; 3], //12bytes data
    pub _pad: f32,     //4bytes padding for alignment
}

impl Globals {
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        Self {
            hsv: [h, s, v],
            _pad: 0.0,
        }
    }
}
#[derive(Clone)]
pub struct Filesave_config {
    pub path: String,
}
pub struct State {
    tx: Sender<DynamicImage>,
    rx: Receiver<DynamicImage>,
    pub skips: u32,
    pub img_vec: VecDeque<DynamicImage>,
    pub img_index: u32,
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub is_surface_configured: bool,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub globals_buffer: wgpu::Buffer,
    pub output_buffer: wgpu::Buffer,
}

impl State {
    pub fn load_image_to_gpu(&mut self) {
        let diffuse_image = self.img_vec.get(self.img_index as usize).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();
        let dimensions = diffuse_image.dimensions();

        self.config.width = dimensions.0;
        self.config.height = dimensions.1;

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let diffuse_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.config.format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("diffuse_texture"),
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diffuse_rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // reuse sampler, layout
        self.diffuse_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.render_pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(
                        &self
                            .device
                            .create_sampler(&wgpu::SamplerDescriptor::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: self.globals_buffer.as_entire_binding(),
                },
            ],
            label: Some("updated_diffuse_bind_group"),
        });
    }

    pub fn draw_to_texture(&mut self, filesave_config: Filesave_config) {
        self.draw(true, Some(filesave_config.clone()));
        console::log_1(&format!("File saved to: {}", filesave_config.path).into());
        self.draw(false, None); // rerender
    }

    pub fn draw(&mut self, update_texture: bool, filesave_config: Option<Filesave_config>) {
        if update_texture {
            self.load_image_to_gpu(); // only use this when image is changed
        }

        // read hsv values
        let hue = use_context::<HSVState>().hue;
        let sat = use_context::<HSVState>().saturation;
        let val = use_context::<HSVState>().value;

        let globals = Globals::new(hue(), sat(), val());
        self.queue
            .write_buffer(&self.globals_buffer, 0, bytemuck::bytes_of(&globals));

        // start rendering the new frame

        let mut frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(err) => {
                console::log_1(&format!("Surface error: {:?}", err).into());
                return;
            }
        };

        let render_target_texture: &Texture;
        let temp_texture: Texture;
        let mut frame_texture = frame.texture.clone();
        let width = self.config.width;
        let height = self.config.height;

        if filesave_config.is_some() {
            temp_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Filesaver Texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                view_formats: &[self.config.format],
            });
            render_target_texture = &temp_texture;
        } else {
            render_target_texture = &frame.texture;
        }

        let view = render_target_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.6,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
        console::log_1(&format!("Prepared frame of size: {}x{}", width, height).into());
        let unpadded_bytes_per_row = 4 * width;
        let padded_bytes_per_row = align_to_256(unpadded_bytes_per_row);
        console::log_1(
            &format!(
                "unpadded_bytes_per_row: {}, padded_bytes_per_row: {}",
                unpadded_bytes_per_row, padded_bytes_per_row
            )
            .into(),
        );
        if filesave_config.is_some() {
            let buffer_size = height * padded_bytes_per_row; // for RGBA8
            self.output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Output Buffer"),
                size: buffer_size as u64,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });

            encoder.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo {
                    texture: &render_target_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyBufferInfo {
                    buffer: &self.output_buffer,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(padded_bytes_per_row),
                        rows_per_image: Some(height),
                    },
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );

            self.queue.submit(Some(encoder.finish()));
            let buffer_length = self.output_buffer.size();
            console::log_1(
                &format!(
                    "Buffer length: {}, should be: {}",
                    buffer_length, buffer_size
                )
                .into(),
            );
            let buffer_slice = self.output_buffer.slice(..);

            let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();

            let output_buffer_clone = self.output_buffer.clone();
            let device_clone = self.device.clone();

            buffer_slice.map_async(wgpu::MapMode::Read, move |res| {
                sender.send(res).unwrap();
            });

            spawn_local(async move {
                if receiver.receive().await.unwrap().is_ok() {
                    let buffer_slice = output_buffer_clone.slice(..);
                    let data = buffer_slice.get_mapped_range();
                    let _ = device_clone.poll(wgpu::PollType::Wait);
                    let mut image_bytes = Vec::with_capacity((width * height * 4) as usize);
                    for y in 0..height {
                        let row_start = (y * padded_bytes_per_row) as usize;
                        let row_end = row_start + unpadded_bytes_per_row as usize;
                        let row = &data[row_start..row_end];
                        image_bytes.extend_from_slice(row);
                    }
                    let buffer =
                        ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, image_bytes).unwrap();

                    save_file_via_dialog(
                        buffer.to_vec(),
                        width,
                        height,
                        filesave_config.unwrap().path,
                    );

                    drop(data);
                    drop(buffer);

                    output_buffer_clone.unmap();
                }
            });
        } else {
            self.queue.submit(Some(encoder.finish()));
            frame.present();
        }
    }

    pub fn sender(&self) -> Sender<DynamicImage> {
        self.tx.clone()
    }

    pub async fn receive(&mut self) {
        loop {
            match self.rx.try_recv() {
                Err(_) => {
                    console::log_1(&"Recieving failed / stopped".into());
                    return;
                }
                Ok(input_file) => {
                    console::log_1(&"File recieved".into());
                    self.img_vec.push_back(input_file);
                }
            };
        }
    }
    pub fn set_index(&mut self, i: u32) {
        if i < self.img_vec.len() as u32 {
            self.img_index = i;
            console::log_1(&format!("Set index to: {}", i).into());
        } else {
            console::log_1(&format!("The index: {}, vec size: {}", i, self.img_vec.len()).into());
            console::log_1(&"The index is out of bounds".into());
        }
    }

    pub async fn new(initial_dyn_image: &DynamicImage) -> State {
        let (tx, rx): (Sender<DynamicImage>, Receiver<DynamicImage>) = mpsc::channel();
        let img_index: u32 = 0;
        let mut img_vec = VecDeque::<DynamicImage>::new();
        img_vec.push_back(initial_dyn_image.clone());
        let diffuse_rgba = img_vec.back().unwrap().to_rgba8();
        use image::GenericImageView;
        let dimensions = img_vec.back().unwrap().dimensions();
        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            depth_or_array_layers: 1,
        };

        let window = window().unwrap();
        let document = window.document().unwrap();

        loop {
            match document.get_element_by_id("image-board").is_some() {
                false => {
                    console::log_1(&"Waiting for canvas".into());
                }
                true => {
                    console::log_1(&"Found canvas".into());
                    break;
                }
            }
        }

        let canvas = document
            .get_element_by_id("image-board")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        let width = canvas.width();
        let height = canvas.height();

        let instance = wgpu::Instance::new(&InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..InstanceDescriptor::default()
        });

        let surface_target = SurfaceTarget::Canvas(canvas);

        let surface = unsafe { instance.create_surface(surface_target).unwrap() };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("No adapter found");

        //Mozilla Firefox fix
        let limits = adapter.limits();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: limits.clone(),
                label: None,
                trace: wgpu::Trace::Off,
                memory_hints: wgpu::MemoryHints::Performance,
            })
            .await
            .unwrap();

        // keep a list of preferred formats here
        let formats = surface.get_capabilities(&adapter).formats;
        let preferred_formats =
            Vec::<TextureFormat>::from([TextureFormat::Rgba8Unorm, TextureFormat::Rgba8UnormSrgb]);
        let mut pref_format = surface.get_capabilities(&adapter).formats[0];

        // select our preferred format from the supported ones
        for cformat in preferred_formats.iter() {
            if formats.contains(cformat) {
                pref_format = *cformat;
                break;
            }
        }

        console::log_1(
            &format!("Formats: {:?}", surface.get_capabilities(&adapter).formats).into(),
        );

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: pref_format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,

            format: pref_format,

            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("diffuse_texture"),

            view_formats: &[],
        });
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &diffuse_rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );
        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let hue = use_context::<HSVState>().hue;
        let sat = use_context::<HSVState>().saturation;
        let val = use_context::<HSVState>().value;

        let globals = Globals::new(hue(), sat(), val());

        let globals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("globals buffer"),
            contents: bytemuck::bytes_of(&globals),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: globals_buffer.as_entire_binding(),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[self::Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,

                unclipped_depth: false,

                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
            cache: None,     // 6.
        });

        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: 0 as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        State {
            tx: tx,
            rx: rx,
            skips: 0,
            img_vec: img_vec,
            img_index: img_index,
            surface: surface,
            device: device,
            queue: queue,
            config: config,
            is_surface_configured: false,
            render_pipeline: render_pipeline,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
            num_indices: num_indices,
            diffuse_bind_group: diffuse_bind_group,
            globals_buffer: globals_buffer.clone(),
            output_buffer: output_buffer,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}
impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2, // NEW!
                },
            ],
        }
    }
}
pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
];

pub const INDICES: &[u16] = &[
    0, 1, 2, //first
    0, 2, 3, //second
];
