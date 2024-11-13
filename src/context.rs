use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::Window};

use egui_wgpu::wgpu;

use crate::{camera::{camera_buffer::CameraBuffer, camera_controller::CameraController, Camera, CameraGraphic, PerspectiveConfig}, renders::RenderManager};


#[allow(unused)]
#[derive(Debug)]
pub struct FcvContext<'window> {
    surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    
    camera: CameraBuffer,
}

impl<'window> FcvContext<'window> {
    pub fn device(&self) -> Arc<wgpu::Device> {
        Arc::clone(&self.device)
    }
    pub fn surface_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.surface_config
    }
    pub fn queue(&self) -> Arc<wgpu::Queue> {
        Arc::clone(&self.queue)
    }
    pub fn camera_group(&self) -> &wgpu::BindGroup {
        self.camera.bind_group()
    }
    pub fn camera_group_layout(&self) -> &wgpu::BindGroupLayout {
        self.camera.layout()
    }

    pub fn new(window: Arc<Window>) -> Self {
        pollster::block_on(Self::new_async(window))
    }

    pub async fn new_async(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(
            window.clone()
        ).expect("Create Surface");
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }
        )
        .await
        .expect("Request Adapter.");
        log::info!("{:?}", adapter.get_info());

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Renderer Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None
            )
            .await
            .expect("Request Device.");
        
        let size = window.inner_size();
        let surface_config = surface
            .get_default_config(&adapter,
                size.width.max(1),
                size.height.max(1),
            ).expect("Setting Surface Config.");
        surface.configure(&device, &surface_config);

        let camera = CameraBuffer::new(
            Camera::default()
                .with_graphic(CameraGraphic::Perspective(
                    PerspectiveConfig {
                        aspect: size.width as f32 / size.height as f32,
                        ..Default::default()
                    }
                ))
            , &device);
        
        Self {
            surface,
            surface_config,
            adapter,
            device: Arc::new(device),
            queue: Arc::new(queue),
            camera,
        }
    }

    pub fn process_camera(&mut self, controller: &mut CameraController) -> bool {
        controller.process_delta(&mut self.camera.camera)
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.surface_config.width = size.width.max(1);
        self.surface_config.height = size.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
        self.camera.camera.on_resize((size.width as f32, size.height as f32));
        self.update_camera_buffer();
    }

    pub fn update_camera_buffer(&mut self) {
        self.camera.update_buffer(&self.queue);
    }

    pub fn render(&mut self, renderer: &mut [&mut dyn RenderManager]) {
        // Get Texture.
        let surface_texture = self.surface.get_current_texture().unwrap();
        // Get Texture View.
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        // Get encoder from device and pass render command.
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        for r in renderer.iter_mut() {
            r.render(&self.device, &mut encoder, &texture_view, &self.camera.bind_group(), &self.queue);
        }

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}