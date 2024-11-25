use std::{cell::RefCell, rc::Rc};

use egui_wgpu::wgpu::{self, util::DeviceExt};
use glam::Mat4;

use crate::shapes::{RenderShape, RenderType};


pub struct ShapeRenderer {
    shape: Rc<RefCell<dyn RenderShape>>,
    matrix: Vec<Mat4>,
    matrix_updated: bool,
    buffer_point: Option<wgpu::Buffer>,
    buffer_color: Option<wgpu::Buffer>,
    buffer_indices: Option<wgpu::Buffer>,
    buffer_instance: Option<wgpu::Buffer>,
}

impl ShapeRenderer {
    pub fn new(render_shape: Rc<RefCell<dyn RenderShape>>) -> Self {
        Self {
            matrix: vec![Mat4::IDENTITY],
            shape: render_shape,
            matrix_updated: true,
            buffer_point: None,
            buffer_color: None,
            buffer_indices: None,
            buffer_instance: None,
        }
    }
    pub fn new_instances(
        render_shape: Rc<RefCell<dyn RenderShape>>,
        transforms: &[Mat4],
    ) -> Self {
        Self {
            shape: render_shape,
            matrix: transforms.to_vec(),
            matrix_updated: true,
            buffer_point: None,
            buffer_color: None,
            buffer_indices: None,
            buffer_instance: None,
        }
    }
    pub fn add_instance(&mut self, mat: Mat4) {
        self.matrix.push(mat);
        self.matrix_updated = true;
    }
    pub fn remove_instance(&mut self, id: usize) {
        self.matrix.remove(id);
        self.matrix_updated = true;
    }
    pub fn set_instances(&mut self, mats: &[Mat4]) {
        self.matrix = mats.to_vec();
        self.matrix_updated = true;
    }
    pub fn get_render_type(&self) -> RenderType {
        self.shape.borrow().get_render_type()
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        pass: &mut wgpu::RenderPass
    ) {
        if self.shape.borrow().should_repaint() || self.matrix_updated
        {
            let pb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: self.shape.borrow().points(),
                    usage: wgpu::BufferUsages::VERTEX| wgpu::BufferUsages::COPY_DST,
                });
            let cb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: self.shape.borrow().colors(),
                    usage: wgpu::BufferUsages::VERTEX| wgpu::BufferUsages::COPY_DST,
                });
            let ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(self.shape.borrow().indices()),
                    usage: wgpu::BufferUsages::INDEX| wgpu::BufferUsages::COPY_DST,
                });
            let inb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&self.matrix),
                    usage: wgpu::BufferUsages::VERTEX| wgpu::BufferUsages::COPY_DST,
                });
    
            self.buffer_point = Some(pb);
            self.buffer_color = Some(cb);
            self.buffer_indices = Some(ib);
            self.buffer_instance = Some(inb);
            self.matrix_updated = false;
        }
        let pb = if let Some(b) = self.buffer_point.as_ref() { b } else { return; };
        let cb = if let Some(b) = self.buffer_color.as_ref() { b } else { return; };
        let ib = if let Some(b) = self.buffer_indices.as_ref() { b } else { return; };
        let inb = if let Some(b) = self.buffer_instance.as_ref() { b } else { return; };
        
        pass.set_vertex_buffer(0, pb.slice(..));
        pass.set_vertex_buffer(1, cb.slice(..));
        pass.set_vertex_buffer(2, inb.slice(..));
        pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.shape.borrow().indices().len() as u32, 0, 0..self.matrix.len() as u32);

        self.shape.borrow_mut().set_repaint(false);
    }
}