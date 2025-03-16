
use wasm_bindgen::prelude::*;
use web_sys::{DomRect, WebGl2RenderingContext, console}; // Исправлено ClientRect -> DomRect
use wgpu::{Instance, SurfaceTarget, Surface, Adapter, Device, Queue, TextureFormat, ShaderModule, RenderPipeline, RenderPassDescriptor, RenderPassColorAttachment, Operations, LoadOp, StoreOp, Color};
use wgpu::util::DeviceExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use bytemuck::{Pod, Zeroable};


#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct Vertex {
    t: f32,
}


#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct ControlPoints {
    points: [f32; 12],
}


#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
struct Params {
    width: f32,
    height: f32,
}

struct Render<'a> {
    surface: Surface<'a>,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,
}


impl<'a> Render<'a> {
    pub async fn new() -> Self {
        let window = web_sys::window().expect("Window");
        let document = window.document().expect("Document");
        let canvas = document
            .get_element_by_id("canvas")
            .expect("Get element")
            .dyn_into::<HtmlCanvasElement>()
            .expect("Dyn into");
    
        // Создаем экземпляр wgpu
        let instance = Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
    
        let width = canvas.client_width() as u32;
        let height = canvas.client_height() as u32;
    
        // Создаем поверхность для рендеринга (исправлено для wasm)
        let surface_target = SurfaceTarget::Canvas(canvas);
        let surface = instance.create_surface(surface_target).expect("Surface");
    
        // Запрашиваем адаптер
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Adapter");
    
        // Запрашиваем устройство и очередь (исправлены поля)
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                    label: None,
                },
                None,
            )
            .await
            .expect("Device");
    
    
        let config = surface.get_default_config(&adapter, width, height).unwrap().to_owned();
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(config.format.into())],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {surface, adapter, device, queue, pipeline}
    }


    pub async fn draw(&self) {
        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&Default::default());
        
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            
            pass.set_pipeline(&self.pipeline);
            pass.draw(0..6, 0..1); // 6 вершин для двух треугольников
        }
        
        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
    
}



#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
  
    let render = Render::new().await;

    render.draw().await;

    

    Ok(())
}


