pub mod detail;
pub mod camera_controller;
pub mod camera_buffer;

use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub enum CameraGraphic {
    Perspective(PerspectiveConfig),
    Orthogonal,
}

#[derive(Debug, Clone, Copy)]
pub struct PerspectiveConfig {
    pub aspect: f32,
    pub fov_y_degree: f32,
}
impl Default for PerspectiveConfig {
    fn default() -> Self {
        Self {
            aspect: 1.,
            fov_y_degree: 45.,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    z_near: f32,
    z_far: f32,
    graphic: CameraGraphic,
    ratio: f32,
}

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}