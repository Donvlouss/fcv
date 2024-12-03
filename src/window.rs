use std::{sync::Arc, time::Instant};

use winit::{dpi::{PhysicalSize, Size}, event::{ElementState, MouseScrollDelta, WindowEvent}, event_loop::EventLoop, platform::windows::EventLoopBuilderExtWindows, window::Window};

use crate::{camera::{camera_controller::{CameraController, CameraEvent}, CameraGraphic, PerspectiveConfig}, context::FcvContext, renders::shape_manager::ShapeManager, ui::EguiRenderer};

const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;

#[derive(Debug, Clone)]
pub struct FcvWindowConfig {
    pub title: String,
    pub update_time: f32,
    pub inner_size: (u32, u32),
    pub camera_rotate_speed: f32,
    pub camera_zoom_speed: f32,
    pub camera_type: CameraGraphic,
}

impl Default for FcvWindowConfig {
    fn default() -> Self {
        Self { 
            title: "Window".to_owned(),
            update_time: 1f32 / 60.,
            inner_size: (DEFAULT_WIDTH, DEFAULT_HEIGHT),
            camera_rotate_speed: 0.01,
            camera_zoom_speed: 0.1,
            camera_type: CameraGraphic::Perspective(
                PerspectiveConfig {
                    aspect: 1.,
                    fov_y_degree: 45.,
                }
            )
        }
    }
}

pub struct FcvWindow<'window> {
    config: FcvWindowConfig,
    window: Option<Arc<Window>>,
    wgpu_context: Option<FcvContext<'window>>,
    camera_controller: CameraController,

    egui_renderer: EguiRenderer,
    shape_manager: ShapeManager,
}

impl<'window> FcvWindow<'window> {
    pub fn new(config: FcvWindowConfig) -> Self {
        Self {
            camera_controller: CameraController::new(config.camera_rotate_speed, config.camera_zoom_speed),
            config, window: None, wgpu_context: None,
            egui_renderer: EguiRenderer::new(),
            shape_manager: ShapeManager::default(),
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
            let ctx  = FcvContext::new(Arc::clone(&window), self.config.camera_type);

            self.shape_manager.build(
                ctx.device(),
                ctx.queue(),
                ctx.surface_config(),
                &[ctx.camera_group_layout()]
            );
            self.egui_renderer.build(
                &ctx.device(), ctx.surface_config().format,
                None, 1, Arc::clone(&window)
            );
            self.wgpu_context = Some(ctx);
            self.window = Some(window);
        }
    }

    pub fn manager(&mut self) -> &mut ShapeManager {
        &mut self.shape_manager
    }

    #[allow(deprecated)]
    pub fn render_loop<F: FnMut(&egui::Context, &mut ShapeManager)>(
            &mut self,
            mut each_frame: F,
    ) {
        let mut timer = Instant::now();
        let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop.run(|e, event_loop| {
                if timer.elapsed().as_secs_f32() >= self.config.update_time {
                    self.window.as_ref().map(|window| window.request_redraw());
                    timer = Instant::now();
                }
            match e {
                winit::event::Event::WindowEvent { window_id: _window_id, event } => {
                    if self.egui_renderer.handle_input(&event) {
                        return;
                    }
                    match event {
                        WindowEvent::CloseRequested => {
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
                            if let (Some(ctx), Some(window)) = (self.wgpu_context.as_mut(), self.window.as_ref()) {

                                if !self.egui_renderer.begin_frame(window) {
                                    return;
                                }
                                self.shape_manager.clear_single();
                                each_frame(
                                    self.egui_renderer.context().as_ref().unwrap(),
                                    &mut self.shape_manager,
                                );
                                ctx.render(
                                    &mut [
                                        &mut self.shape_manager,
                                        &mut self.egui_renderer,
                                    ]
                                );
                            }
                        }
                        _ => {}
                    }
                    if let Some(ctx) = self.wgpu_context.as_mut() {
                        if ctx.process_camera(&mut self.camera_controller) {
                            ctx.update_camera_buffer();
                        }
                    }
                },
                winit::event::Event::Resumed => {
                    self.create_window(&event_loop);
                },
                winit::event::Event::AboutToWait => {
                    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
                },
                winit::event::Event::LoopExiting => {
                    self.wgpu_context = None;
                },
                _ => {}
            }
        }).unwrap();
    }
}
