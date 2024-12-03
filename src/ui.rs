use std::sync::Arc;

use egui::Context;
use egui_wgpu::wgpu::{Device, StoreOp, TextureFormat};
use egui_wgpu::{wgpu, Renderer, ScreenDescriptor};
use egui_winit::State;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::renders::RenderManager;

/// Ref: [egui_tools.rs](https://github.com/kaphula/winit-egui-wgpu-template/blob/master/src/egui_tools.rs)
pub struct EguiRenderer {
    state: Option<State>,
    renderer: Option<Renderer>,
    frame_started: bool,
    window: Option<Arc<Window>>
}

impl EguiRenderer {
    pub fn context(&self) -> Option<&Context> {
        match self.state.as_ref() {
            Some(state) => Some(state.egui_ctx()),
            None => None,
        }
    }
    pub fn new() -> Self {
        Self {
            state: None,
            renderer: None,
            frame_started: false,
            window: None,
        }
    }

    pub fn build(
        &mut self,
        device: &Device,
        output_color_format: TextureFormat,
        output_depth_format: Option<TextureFormat>,
        msaa_samples: u32,
        window: Arc<Window>,
    ) {
        let egui_context = Context::default();

        let egui_state = egui_winit::State::new(
            egui_context,
            egui::viewport::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            Some(2 * 1024), // default dimension is 2048
        );
        let egui_renderer = Renderer::new(
            device,
            output_color_format,
            output_depth_format,
            msaa_samples,
            true,
        );

        self.state = Some(egui_state);
        self.renderer = Some(egui_renderer);
        self.frame_started = false;
        self.window = Some(window);
    }

    pub fn handle_input(&mut self, event: &WindowEvent) -> bool {
        if let (Some(state), Some(window)) = (self.state.as_mut(), self.window.as_ref()) {
            let r = state.on_window_event(window, event);
            r.consumed
        } else {
            false
        }
    }

    pub fn ppp(&mut self, v: f32) {
        if let Some(state) = self.state.as_mut() {
            state.egui_ctx().set_pixels_per_point(v);
        }
    }

    pub fn begin_frame(&mut self, window: &Window) -> bool {
        if let Some(state) = self.state.as_mut() {
            let raw_input = state.take_egui_input(window);
            state.egui_ctx().begin_pass(raw_input);
            self.frame_started = true;
            true
        } else {
            false
        }
    }
}

impl RenderManager for EguiRenderer {
    fn render(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        _bind_group: &wgpu::BindGroup,
        _depth_view: &wgpu::TextureView,
        queue: &wgpu::Queue,
    ) {
        if !self.frame_started {
            return;
        }
        let desc = {   
            let window = if let Some(window) = self.window.as_ref() {
                window
            } else { return; };
            let desc = ScreenDescriptor {
                size_in_pixels: [
                    window.inner_size().width,
                    window.inner_size().height,
                ],
                pixels_per_point: window.scale_factor() as f32
            };
            self.ppp(desc.pixels_per_point);
            desc
        };

        let (state, renderer, window) = 
            if let (Some(state), Some(r), Some(window)) = 
            (self.state.as_mut(), self.renderer.as_mut(), self.window.as_ref())
        { (state, r, window) } else { return; };

        let full_output = state.egui_ctx().end_pass();

        state
            .handle_platform_output(&window, full_output.platform_output);

        let tris = state
            .egui_ctx()
            .tessellate(full_output.shapes, state.egui_ctx().pixels_per_point());
        for (id, image_delta) in &full_output.textures_delta.set {
            renderer
                .update_texture(device, queue, *id, image_delta);
        }
        renderer
            .update_buffers(device, queue, encoder, &tris, &desc);
        let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: view,
                resolve_target: None,
                ops: egui_wgpu::wgpu::Operations {
                    load: egui_wgpu::wgpu::LoadOp::Load,
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            label: Some("egui main render pass"),
            occlusion_query_set: None,
        });

        renderer
            .render(&mut rpass.forget_lifetime(), &tris, &desc);
        for x in &full_output.textures_delta.free {
            renderer.free_texture(x)
        }

        self.frame_started = false;
    }
}