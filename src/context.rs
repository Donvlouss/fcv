use std::sync::Arc;
use winit::window::Window;



#[derive(Debug)]
pub struct FcvContext<'window> {
    surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl<'window> FcvContext<'window> {
    pub async fn new(window: Arc<Window>) -> Self {
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


        Self {
            surface,
            surface_config,
            adapter,
            device,
            queue
        }
    }
}