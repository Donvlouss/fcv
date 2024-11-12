mod handler;

use std::sync::Arc;

use glam::{Vec3, Vec4};
use winit::window::Window;

use crate::{camera::camera_controller::CameraController, context::FcvContext};

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
}

impl<'window> FcvWindow<'window> {
    pub fn new(config: FcvWindowConfig) -> Self {
        Self {
            config, window: None, wgpu_context: None,
            camera_controller: CameraController::default()
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
    ) -> Option<usize> {
        if let Some(ctx) = self.wgpu_context.as_mut() {
            Some(ctx.add_points_with_indices(points, colors, indices))
        } else {
            None
        }
    }
    pub fn add_points(
        &mut self,
        points: &[Vec3],
        colors: &[Vec4],
    ) -> Option<usize> {
        if let Some(ctx) = self.wgpu_context.as_mut() {
            Some(ctx.add_points(points, colors))
        } else {
            None
        }
    }
    pub fn add_points_uniform_color(
        &mut self,
        points: &[Vec3],
        color: Vec4,
    ) -> Option<usize> {
        if let Some(ctx) = self.wgpu_context.as_mut() {
            Some(ctx.add_points_uniform_color(points, color))
        } else {
            None
        }
    }
    pub fn remove_points(&mut self, id: usize) {
        if let Some(ctx) = self.wgpu_context.as_mut() {
            Some(ctx.remove_points(id));
        } 
    }
    pub fn draw_point(&mut self, p: Vec3, c: Vec4) {
        if let Some(ctx) = self.wgpu_context.as_mut() {
            Some(ctx.draw_point(p, c));
        }
    }
}