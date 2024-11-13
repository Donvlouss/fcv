use std::{sync::Arc, time::{Duration, Instant}};

use glam::{Vec3, Vec4};
use winit::{dpi::{PhysicalSize, Size}, event::{ElementState, MouseScrollDelta, WindowEvent}, event_loop::EventLoop, window::Window};

use crate::{camera::camera_controller::{CameraController, CameraEvent}, context::FcvContext, renders::vertex_manager::VertexManager};

#[derive(Debug, Default, Clone, Copy)]
pub enum WindowUpdateMode {
    Immediately,
    StaticTime(f32),
    #[default]
    WaitEvent,
}

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

#[derive(Debug, Clone)]
pub struct FcvWindowConfig {
    pub title: String,
    pub mode: WindowUpdateMode,
    pub inner_size: (u32, u32),
}

impl Default for FcvWindowConfig {
    fn default() -> Self {
        Self { title: "Window".to_owned(), mode: Default::default(), inner_size: (DEFAULT_WIDTH, DEFAULT_HEIGHT) }
    }
}

#[derive(Debug)]
pub struct FcvWindow<'window> {
    config: FcvWindowConfig,
    window: Option<Arc<Window>>,
    wgpu_context: Option<FcvContext<'window>>,
    camera_controller: CameraController,

    vertex_render: VertexManager,
}

impl<'window> FcvWindow<'window> {
    pub fn new(config: FcvWindowConfig) -> Self {
        Self {
            config, window: None, wgpu_context: None,
            camera_controller: CameraController::new(0.1),
            vertex_render: VertexManager::default()
        }
    }

    fn create_window(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let attribute = Window::default_attributes()
                .with_title(&self.config.title)
                .with_inner_size(Size::Physical(PhysicalSize::new(self.config.inner_size.0.max(1), self.config.inner_size.1.max(1))));
            let window = Arc::new(event_loop.create_window(attribute).expect("Create Window."));
            self.camera_controller.resize(
                (window.inner_size().width, window.inner_size().height)
            );
            let ctx  = FcvContext::new(window.clone());
            self.vertex_render.build(
                ctx.device(),
                ctx.queue(),
                ctx.surface_config(),
                &[ctx.camera_group_layout()]
            );
            self.wgpu_context = Some(ctx);
            self.window = Some(window);
        }
    }

    #[allow(deprecated)]
    pub fn render_loop<F: FnMut(&mut VertexManager)>(&mut self, event_loop: EventLoop<()>, mut each_frame: F) {
        event_loop.run(|e, event_loop| {
            match e {
                winit::event::Event::WindowEvent { window_id: _window_id, event } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            println!("Request Exited");
                            event_loop.exit();
                        },
                        WindowEvent::Resized(size) => {
                            if let Some(ctx) = self.wgpu_context.as_mut() {
                                ctx.resize(size);
                                self.camera_controller.resize((size.width, size.height));
                            }
                        },
                        WindowEvent::MouseInput { device_id: _device_id, state, button } => {
                            if ElementState::Pressed == state {
                                match button {
                                    winit::event::MouseButton::Left => {
                                        self.camera_controller.enable_event(CameraEvent::Rot);
                                    },
                                    winit::event::MouseButton::Right => {
                                        self.camera_controller.enable_event(CameraEvent::Pan);
                                    },
                                    _ => {}
                                }
                            } else {
                                self.camera_controller.disable_event();
                            }
                            
                        },
                        WindowEvent::MouseWheel { device_id: _device_id, delta, phase: _phase } => {
                            if let MouseScrollDelta::LineDelta(x, y) = delta {
                                self.camera_controller.enable_event(CameraEvent::Zoom);
                                self.camera_controller.set_zoom_delta((x, y));
                            }
                        },
                        WindowEvent::CursorMoved { device_id: _device_id, position } => {
                            let pos = (position.x, position.y);
                            self.camera_controller.set_pos(pos);
                            if self.camera_controller.enabled() {
                                self.camera_controller.set_delta(pos);
                            }
                        },
                        WindowEvent::RedrawRequested => {
                            if let Some(ctx) = self.wgpu_context .as_mut() {
                                each_frame(&mut self.vertex_render);
                                // ctx.render(vec![&mut self.vertex_render]);
                                ctx.render(&mut [&mut self.vertex_render]);
                            }
                        }
                        _ => {}
                    }
                    if let (Some(ctx), Some(window)) = (self.wgpu_context.as_mut(), self.window.as_ref()) {
                        if ctx.process_camera(&mut self.camera_controller) {
                            ctx.update_camera_buffer();
                            window.request_redraw();
                        }
                    }
                },
                winit::event::Event::Resumed => {
                    self.create_window(&event_loop);
                },
                winit::event::Event::AboutToWait => {
                    event_loop.set_control_flow(
                        match self.config.mode {
                            WindowUpdateMode::Immediately => winit::event_loop::ControlFlow::Poll,
                            WindowUpdateMode::StaticTime(delta) => {
                                winit::event_loop::ControlFlow::WaitUntil(
                                    Instant::now() + Duration::from_secs_f32(delta)
                                )
                            },
                            WindowUpdateMode::WaitEvent => winit::event_loop::ControlFlow::Wait,
                        }
                    );
                    if let Some(window) = self.window.as_ref() {
                        window.request_redraw();
                    }
                },
                winit::event::Event::LoopExiting => {
                    self.wgpu_context = None;
                },
                _ => {}
            }
        }).unwrap();
    }
}

// Vertex
impl<'window> FcvWindow<'window> {
    pub fn add_points_with_indices(
        &mut self,
        points: &[Vec3],
        colors: &[Vec4],
        indices: &[u32]
    ) -> usize {
        self.vertex_render.add_points_with_indices(points, colors, indices)
    }
    pub fn add_points(
        &mut self,
        points: &[Vec3],
        colors: &[Vec4],
    ) -> usize {
        self.vertex_render.add_points(points, colors)
    }
    pub fn add_points_uniform_color(
        &mut self,
        points: &[Vec3],
        color: Vec4,
    ) -> usize {
        self.vertex_render.add_points_uniform_color(points, color)
    }
    pub fn remove_points(&mut self, id: usize) {
        self.vertex_render.remove_item(id);
    }
}