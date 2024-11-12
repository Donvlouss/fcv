mod manages;

use std::sync::Arc;
use winit::{dpi::PhysicalSize, window::Window};

use crate::{camera::{camera_buffer::CameraBuffer, camera_controller::CameraController, Camera, CameraGraphic, PerspectiveConfig}, renders::vertex_manager::VertexManager};



#[derive(Debug)]
pub struct FcvContext<'window> {
    surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    
    camera: CameraBuffer,

    vertex_manager: VertexManager,
}

impl<'window> FcvContext<'window> {
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

        let vertex_manager = VertexManager::new(&device, &surface_config, &[camera.layout()]);
        
        Self {
            surface,
            surface_config,
            adapter,
            device,
            queue,
            camera,
            vertex_manager
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

}