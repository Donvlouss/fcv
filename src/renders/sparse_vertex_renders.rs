use glam::{Vec3, Vec4};
use wgpu::util::DeviceExt;

use crate::buffers::vertex_buffer::PointColorBuffer;


#[derive(Debug)]
pub struct SparseVertexRender {
    data: Vec<PointColorBuffer>,
    buffer_data: Option<wgpu::Buffer>,
    buffer_indices: Option<wgpu::Buffer>,
}

impl SparseVertexRender {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(100),
            buffer_data: None,
            buffer_indices: None
        }
    }
    pub fn add(
        &mut self,
        p: Vec3,
        c: Vec4
    ) {
        self.data.push(
            PointColorBuffer {
                point: p.into(),
                color: c.into(),
            }
        );
    }
    pub fn render(&mut self,
        device: &wgpu::Device,
        pass: &mut wgpu::RenderPass) {
        let indices = (0..self.data.len() as u32).collect::<Vec<u32>>();
        let buffer_data = 
            device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Sparse Vertex Buffer")),
                    contents: bytemuck::cast_slice(&self.data),
                    usage: wgpu::BufferUsages::VERTEX,
                }
            );
        let buffer_indices = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Sparse Vertex Index Buffer")),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        
        pass.set_vertex_buffer(0, buffer_data.slice(..));
        pass.set_index_buffer(buffer_indices.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.data.len() as u32, 0, 0..1);

        self.data.clear();
        self.buffer_data = Some(buffer_data);
        self.buffer_indices = Some(buffer_indices);
    }
}