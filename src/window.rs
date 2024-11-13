mod handler;

use std::sync::Arc;

use glam::{Vec3, Vec4};
use winit::window::Window;

use crate::{camera::camera_controller::CameraController, context::FcvContext, renders::{vertex_manager::VertexManager, vertex_renders::VertexBuffer}};

#[derive(Debug, Clone, Default)]
pub struct FcvWindowConfig {
    pub title: String,
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

    pub fn render(&self) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
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
        let id = self.vertex_render.request_index();
        let (d, q) = if let Some(ctx) = self.wgpu_context.as_mut() {
            let device = ctx.device();
            let queue = ctx.queue();
            (Some(device), Some(queue))
        } else {
            (None, None)
        };
        let buffer = VertexBuffer::build_from_pci(id,
            bytemuck::cast_slice(points),
            bytemuck::cast_slice(colors),
            bytemuck::cast_slice(indices),
            d, q);
        self.vertex_render.add_item(buffer, Some(id));
        id
    }
    pub fn add_points(
        &mut self,
        points: &[Vec3],
        colors: &[Vec4],
    ) -> usize {
        let indices = (0..(points.len().min(colors.len()) as u32)).collect::<Vec<_>>();
        self.add_points_with_indices(points, colors, &indices)
    }
    pub fn add_points_uniform_color(
        &mut self,
        points: &[Vec3],
        color: Vec4,
    ) -> usize {
        let colors = vec![color; points.len()];
        self.add_points(points, &colors)
    }
    pub fn remove_points(&mut self, id: usize) {
        self.vertex_render.remove_item(id);
    }
    pub fn draw_point(&mut self, p: Vec3, c: Vec4) {
        self.vertex_render.draw_point(p, c);
    }
}