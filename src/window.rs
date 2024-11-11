use std::sync::Arc;

use winit::{application::ApplicationHandler, window::Window};

#[derive(Debug, Clone, Default)]
pub struct FcvWindowConfig {
    pub title: String,
}

#[derive(Debug, Default)]
pub struct FcvWindow {
    config: FcvWindowConfig,
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for FcvWindow {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let attribute = Window::default_attributes().with_title(&self.config.title);
            let window = Arc::new(event_loop.create_window(attribute).expect("Create Window."));
            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
        
    }
}