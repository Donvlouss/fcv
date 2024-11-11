use wgpu::util::DeviceExt;

use super::{Camera, CameraUniform};


#[derive(Debug)]
pub struct CameraBuffer {
    pub camera: Camera,
    buffer: wgpu::Buffer
}

impl CameraBuffer {
    pub fn new(
        camera: Camera,
        device: &wgpu::Device
    ) -> Self {
        Self {
            camera,
            buffer: device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Buffer"),
                    contents: bytemuck::cast_slice(&[CameraUniform::from(camera)]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                }
            )
        }
    }

    pub fn update_buffer(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0,
            bytemuck::cast_slice(&[CameraUniform::from(self.camera)]));
    }
}