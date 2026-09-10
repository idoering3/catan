use std::sync::Arc;

use wgpu::{util::DeviceExt};
use winit::{
    application::ApplicationHandler, event::*, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::Window
};

// the vertex struct for an individual vertex
// each vertex has a position and rgb color
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
// vertex, next vertex, position, color, offset. two strides.
impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}

// lib.rs
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
];

// challenge code - complex shape (more than 3 triangles)
const HEXAGON_VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, 0.866, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-1.0, 0.0, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.5, -0.866, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.5, -0.866, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [1.0, 0.0, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.5, 0.866, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.0, 0.0, 0.0], color: [0.5, 0.0, 0.5] },
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

const HEXAGON_INDICES: &[u16] = &[
    0, 1, 6,
    1, 2, 6,
    2, 3, 6,
    3, 4, 6,
    4, 5, 6,
    5, 0, 6,
];


// This will store the state of our game
// Manages the graphics/game state
// owns graphics stuff
pub struct State {
    // reference can live for the entire duration of the program
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    // the actual application window that is displayed.
    window: Arc<Window>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    
    // variables used for challenges
    challenge_shape_enabled: bool,
    vertex_buffer_hex: wgpu::Buffer,
    index_buffer_hex: wgpu::Buffer,
    num_indices_hex: u32,
}

impl State {
    // We don't need this to be async right now,
    // but we will in the next tutorial
    async fn new(window: Arc<Window>) -> anyhow::Result<State> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: true,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        };

        // define the shader, we get the shaders from shader.wgsl file
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // what external resources to these shaders need?
        // e.g.
        /*
            Camera
            Textures
            Samplers
            Lighting
            Uniforms
         */
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                immediate_size: 0,
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                // THIS IS OUR VERTEX SHADER
                entry_point: Some("vs_main"), // 1.
                buffers: &[
                    // use the vertex buffer desc function.
                    // each entry is a slot the buffer can be bound to
                    // None means the slot is empty
                    // e.g. we can have material, vertex, instance buffers
                    Some(Vertex::desc())
                ], // 2.
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                // THIS IS OUR FRAGMENT SHADER
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            // How do we interpret our vertices when converting them into triangles?h
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1. Means every 3 vertices is one triangle
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2. Determine whether triangle is facing forward or not
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            // continued ...
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview_mask: None, // 5.
            cache: None, // 6.
        });


        // create a pile of vertices that will be sent to the gpu. Contains our vertices
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let num_indices = INDICES.len() as u32;
        
        //challenge code for the hexagon:
        let vertex_buffer_hex = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(HEXAGON_VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer_hex = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(HEXAGON_INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_indices_hex = HEXAGON_INDICES.len() as u32;


        let challenge_shape_enabled = false;

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            render_pipeline,
            window,
            vertex_buffer,
            index_buffer,
            num_indices,

            challenge_shape_enabled,
            vertex_buffer_hex,
            index_buffer_hex,
            num_indices_hex
        })
    }

    // impl State
    pub fn resize(&mut self, width: u32, height: u32) {
        // NOTE: max supported dimension in WebGL is 2048px
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config); 
            self.is_surface_configured = true;
        }
    }

    // impl State
    // handles keyboard events
    fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            // If space pressed, change the variable for using color pipeline or not...
            (KeyCode::Space, true) => {
                // do something when a space is pressed here!
                self.challenge_shape_enabled =!self.challenge_shape_enabled;
                self.window.request_redraw();
            }
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }

    // handle mouse moved changes the clear color to a value that makes sense based on the mouse position.
    // TODO: have window_size as a paramter in State
    fn handle_mouse_moved(&mut self) {
        // get the dimensions of the window, because position
        // is relative to the window
        // let window_size = self.window.inner_size();


        self.window.request_redraw();
    }

    fn update(&mut self) {
        // remove 'todo!()'
    }
    
    pub fn render(&mut self) -> anyhow::Result<()>{
        self.window.request_redraw();

        // can't render until the surface is configured.
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                surface_texture
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                // Skip this frame
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                // You could recreate the devices and all resources
                // created with it here, but we'll just bail
                anyhow::bail!("Lost device");
            }
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // the enclosure is to drop the encoder borrow on begin_render_pass()
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }
                            ),
                            store: wgpu::StoreOp::Store,
                        }
                    })
                ],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            // tell wgpu to use the vertex buffer and to draw the vertices in the buffer

            //challenge code: check the variable to see if we draw the regular shape or a hexagon
            render_pass.set_pipeline(&self.render_pipeline);
            if self.challenge_shape_enabled {
                render_pass.set_vertex_buffer(0, self.vertex_buffer_hex.slice(..));
                render_pass.set_index_buffer(self.index_buffer_hex.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..self.num_indices_hex, 0, 0..1); // 3.
            } else {
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..self.num_indices, 0, 0..1); // 3.
            }
        }

        // submit will accept anything that implements IntoIter
        // submits the buffer to the GPU render queue
        // encoder.finish turns our encoder into a CommandBuffer
        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(output);

        Ok(())
    }
}

// app struct
// manages the application lifecycle
// receives events from OS
pub struct App {
    // state is an option because the APPLICATION exists BEFORE the state exists
    state: Option<State>,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: None,
        }
    }
}

impl ApplicationHandler<State> for App {
    // function is essentially "ok, you're allowed to initialize the app now"
    // main() -> create App -> event_loop.run_app() -> resumed() -> create window -> render loop
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            // If we are not on web we can use pollster to
            // await the window creation
            self.state = Some(pollster::block_on(State::new(window)).unwrap());
        }
    }

    // for delivering custom data/event back to my App.
    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        // This is where proxy.send_event() ends up
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(e) => {
                        // Log the error and exit gracefully
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::CursorMoved { 
                device_id: _, position: _ 
            } => state.handle_mouse_moved(),
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),
            _ => {}
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }

    let event_loop = EventLoop::with_user_event().build()?;
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut app = App::new();
        event_loop.run_app(&mut app)?;
    }

    Ok(())
}