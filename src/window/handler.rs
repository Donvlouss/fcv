use std::sync::Arc;

use winit::{application::ApplicationHandler, event::{ElementState, MouseScrollDelta, WindowEvent}, window::Window};

use crate::{camera::camera_controller::CameraEvent, context::FcvContext};

use super::FcvWindow;


impl<'window> ApplicationHandler for FcvWindow<'window> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let attribute = Window::default_attributes().with_title(&self.config.title);
            let window = Arc::new(event_loop.create_window(attribute).expect("Create Window."));
            self.camera_controller.resize(
                (window.inner_size().width, window.inner_size().height)
            );
            let ctx  = FcvContext::new(window.clone());
            self.vertex_render.build(ctx.device(), ctx.surface_config(), &[ctx.camera_group_layout()]);

            self.wgpu_context = Some(ctx);
            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
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
                if let Some(ctx) = self.wgpu_context .as_mut() {
                    // ctx.render(vec![&mut self.vertex_render]);
                    ctx.render(&mut [&mut self.vertex_render]);
                }
            }
            _ => {}
        };
        if let (Some(ctx), Some(window)) = (self.wgpu_context.as_mut(), self.window.as_ref()) {
            if ctx.process_camera(&mut self.camera_controller) {
                ctx.update_camera_buffer();
                window.request_redraw();
            }
        }
    }
}
