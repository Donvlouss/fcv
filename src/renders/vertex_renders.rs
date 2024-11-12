use wgpu::util::DeviceExt;

use crate::buffers::{vertex_buffer::{ColorBuffer, PointBuffer}, BufferType};

const INDEX_SIZE: u64 = std::mem::size_of::<u32>() as u64;

#[derive(Debug)]
pub struct VertexBuffer {
    index: usize,
    buffer_point: Option<wgpu::Buffer>,
    buffer_color: Option<wgpu::Buffer>,
    buffer_indices: Option<wgpu::Buffer>,
}

impl VertexBuffer {
    pub fn new(index: usize) -> Self {
        Self { index, buffer_point: None, buffer_color: None, buffer_indices: None }
    }
    pub fn set_points(
        mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        points: &[PointBuffer]
    ) -> Self {
        let need_create = if self.buffer_point.is_some() {
            points.len() as u64 * PointBuffer::size() != self.buffer_point.as_ref().unwrap().size()
        } else {
            true
        };
        if need_create {
            self.buffer_point = Some(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Vertex Buffer @ {}", self.index)),
                    contents: bytemuck::cast_slice(points),
                    usage: wgpu::BufferUsages::VERTEX,
                }
            ));
        } else {
            if let Some(buff) = &self.buffer_point {
                queue.write_buffer(
                    &buff, 0, bytemuck::cast_slice(points));
            }
        }
        self
    }
    pub fn set_colors(
        mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        colors: &[ColorBuffer]
    ) -> Self {
        let need_create = if self.buffer_color.is_some() {
            colors.len() as u64 * ColorBuffer::size() != self.buffer_color.as_ref().unwrap().size()
        } else {
            true
        };
        if need_create {
            self.buffer_color = Some(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Vertex Buffer @ {}", self.index)),
                    contents: bytemuck::cast_slice(colors),
                    usage: wgpu::BufferUsages::VERTEX,
                }
            ));
        } else {
            if let Some(buff) = &self.buffer_color {
                queue.write_buffer(
                    &buff, 0, bytemuck::cast_slice(colors));
            }
        }
        self
    }
    pub fn set_indices(
        mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        indices: &[u32]
    ) -> Self {
        let need_create = if self.buffer_indices.is_some() {
            indices.len() as u64 * INDEX_SIZE != self.buffer_indices.as_ref().unwrap().size()
        } else {
            true
        };
        if need_create {
            self.buffer_indices = Some(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Vertex Buffer @ {}", self.index)),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                }
            ));
        } else {
            if let Some(buff) = &self.buffer_indices {
                queue.write_buffer(
                    buff, 0, bytemuck::cast_slice(indices));
            }
        }
        self
    }
    pub fn points_buffer(&self) -> Option<&wgpu::Buffer> {
        if let Some(b) = &self.buffer_point {
            Some(b)
        } else {
            None
        }
    }
    pub fn colors_buffer(&self) -> Option<&wgpu::Buffer> {
        if let Some(b) = &self.buffer_color {
            Some(b)
        } else {
            None
        }
    }
    pub fn indices_buffer(&self) -> Option<&wgpu::Buffer> {
        if let Some(b) = &self.buffer_indices {
            Some(b)
        } else {
            None
        }
    }

    fn check(&self) -> bool {
        let (c, i) = match (
            &self.buffer_point, &self.buffer_color, &self.buffer_indices
        ) {
            (Some(p), Some(c), Some(i)) => {
                (
                    p.size() / PointBuffer::size() ==c.size() / ColorBuffer::size(),
                    i.size() != 0 && p.size() / PointBuffer::size() == i.size() / INDEX_SIZE
                )
            },
            (Some(p), None, Some(i)) => {
                (
                    false,
                    i.size() != 0 && p.size() / PointBuffer::size() == i.size() / INDEX_SIZE
                )
            },
            (Some(p), Some(c), None) => {
                (
                    p.size() / PointBuffer::size() == c.size() / ColorBuffer::size(),
                    false
                )
            }
            _ => {(false, false)}
        };
        c && i
    }
    pub fn render(&self, pass: &mut wgpu::RenderPass) {
        if !self.check() {
            return;
        }
        if let (
            Some(p), Some(c), Some(i)
        ) = (&self.buffer_point, &self.buffer_color, &self.buffer_indices) {
            pass.set_vertex_buffer(0, p.slice(..));
            pass.set_vertex_buffer(1, c.slice(..));
            pass.set_index_buffer(i.slice(..), wgpu::IndexFormat::Uint32);
            let nb_indices = (i.size() / INDEX_SIZE) as u32;
            pass.draw_indexed(0..nb_indices, 0, 0..1);
        }

    }
}