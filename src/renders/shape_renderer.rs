use std::{cell::RefCell, rc::Rc};

use egui_wgpu::wgpu::{self, util::DeviceExt};

use crate::shapes::{RenderShape, RenderType};


pub struct ShapeRenderer {
    shape: Rc<RefCell<dyn RenderShape>>,
    matrix: Vec<[[f32; 4]; 4]>,
    matrix_updated: bool,
    buffer_point: Option<wgpu::Buffer>,
    buffer_color: Option<wgpu::Buffer>,
    buffer_indices: Option<wgpu::Buffer>,
    buffer_instance: Option<wgpu::Buffer>,
}

impl ShapeRenderer {
    pub fn new(render_shape: Rc<RefCell<dyn RenderShape>>) -> Self {
        let mat = render_shape.borrow().transform();
        Self {
            matrix: vec![mat],
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
        transforms: &[[[f32; 4]; 4]],
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
    pub fn add_matrix(&mut self, mat: [[f32; 4]; 4]) {
        self.matrix.push(mat);
        self.matrix_updated = true;
    }
    pub fn remove_matrix(&mut self, id: usize) {
        self.matrix.remove(id);
        self.matrix_updated = true;
    }
    pub fn set_matrixes(&mut self, mats: &[[[f32; 4]; 4]]) {
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
        if !self.shape.borrow().should_repaint()
            || !self.matrix_updated {
            return;
        }
        let pb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(self.shape.borrow().points()),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let cb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(self.shape.borrow().colors()),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let n_indices = self.shape.borrow().indices().len() as u32;
        let ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(self.shape.borrow().indices()),
                usage: wgpu::BufferUsages::INDEX,
            });
        let inb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&self.matrix),
                usage: wgpu::BufferUsages::VERTEX,
            });

        pass.set_vertex_buffer(0, pb.slice(..));
        pass.set_vertex_buffer(1, cb.slice(..));
        pass.set_vertex_buffer(2, inb.slice(..));
        pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..n_indices, 0, 0..self.matrix.len() as u32);

        self.buffer_point = Some(pb);
        self.buffer_color = Some(cb);
        self.buffer_indices = Some(ib);
        self.buffer_instance = Some(inb);
    }
}