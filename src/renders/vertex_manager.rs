use std::collections::HashMap;

use glam::{Vec3, Vec4};
use wgpu::{include_wgsl, RenderPassDescriptor};

use crate::{buffers::{vertex_buffer::{ColorBuffer, PointBuffer, PointColorBuffer}, BufferType}, create_pipeline};

use super::{sparse_vertex_renders::SparseVertexRender, vertex_renders::VertexBuffer, RenderManager};


#[derive(Debug, Default)]
pub struct VertexManager {
    map: HashMap<usize, VertexBuffer>,
    sparse: SparseVertexRender,
    counter: usize,
    pipeline: Option<wgpu::RenderPipeline>,
    sparse_pipeline: Option<wgpu::RenderPipeline>,
}

impl VertexManager {
    pub fn build(
        &mut self,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) {
        let shaders = device.create_shader_module(include_wgsl!("../shaders/points.wgsl"));
        let layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Points Layout"),
                bind_group_layouts,
                push_constant_ranges: &[],
            }
        );
        let sparse_pipeline = create_pipeline!(
            device, config, bind_group_layouts,
            shaders, layout, "Points Pipeline",
            &[PointColorBuffer::desc()],
            wgpu::PrimitiveTopology::PointList
        );
        let pipeline = create_pipeline!(
            device, config, bind_group_layouts,
            shaders, layout, "Points Pipeline",
            &[PointBuffer::desc(), ColorBuffer::desc()],
            wgpu::PrimitiveTopology::PointList
        );
        self.pipeline = Some(pipeline);
        self.sparse_pipeline = Some(sparse_pipeline);
    }
    pub fn request_index(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }
    pub fn add_item(&mut self, buffer: VertexBuffer, id: Option<usize>) {
        let id = match id {
            Some(id) => id,
            None => self.request_index(),
        };
        self.map.entry(id).or_insert(buffer);
    }
    pub fn remove_item(&mut self, id: usize) {
        if self.map.contains_key(&id) {
            self.map.remove(&id);
        }
    }
    pub fn entry(&self, id: &usize) -> Option<&VertexBuffer> {
        self.map.get(id)
    }
    pub fn entry_mut(&mut self, id: &usize) -> Option<&mut VertexBuffer> {
        self.map.get_mut(id)
    }
    pub fn draw_point(&mut self, p: Vec3, c: Vec4) {
        self.sparse.add(p, c);
    }
    fn render_sparse(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        bind_group: &wgpu::BindGroup
    ) {
        let pipeline = if let Some(p) = self.sparse_pipeline.as_ref() {
            p
        } else {
            return;
        };
        let mut pass = encoder.begin_render_pass(
            &RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            }
        );
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        self.sparse.render(device, &mut pass);
    }
    fn render_multiple(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        bind_group: &wgpu::BindGroup,
    ) {
        let pipeline = if let Some(p) = self.pipeline.as_ref() {
            p
        } else {
            return;
        };
        let mut pass = encoder.begin_render_pass(
            &RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            }
        );
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        for buff in self.map.values_mut() {
            buff.render(&mut pass, device);
        }
    }
}

impl RenderManager for VertexManager {
    fn render(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        bind_group: &wgpu::BindGroup,
        _queue: &wgpu::Queue
    ) {
        self.render_multiple(device, encoder, view, bind_group);
        self.render_sparse(device, encoder, view, bind_group);
    }
}