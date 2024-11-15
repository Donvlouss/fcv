
use std::{collections::HashMap, sync::Arc};

use egui_wgpu::wgpu::{self, include_wgsl};

use crate::{buffers::{transform_buffer::TransformBuffer, vertex_buffer::{ColorBuffer, PointBuffer}, BufferType}, create_pipeline, shapes::RenderType};

use super::{shape_renderer::ShapeRenderer, RenderManager};

#[derive(Default)]
pub struct ShapeManager {
    map: HashMap<usize, ShapeRenderer>,
    counter: usize,

    pl_points: Option<wgpu::RenderPipeline>,
    pl_line: Option<wgpu::RenderPipeline>,
    pl_line_strip: Option<wgpu::RenderPipeline>,
    pl_tri: Option<wgpu::RenderPipeline>,
    pl_tri_strip: Option<wgpu::RenderPipeline>,

    device: Option<Arc<wgpu::Device>>,
    queue: Option<Arc<wgpu::Queue>>,
}

impl ShapeManager {
    pub fn build(
        &mut self,
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
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
        self.pl_points = Some(create_pipeline!(
            device, config, bind_group_layouts,
            shaders, layout, "Points Pipeline",
            &[PointBuffer::desc(), ColorBuffer::desc(), TransformBuffer::desc()],
            wgpu::PrimitiveTopology::PointList
        ));
        self.pl_line = Some(create_pipeline!(
            device, config, bind_group_layouts,
            shaders, layout, "Points Pipeline",
            &[PointBuffer::desc(), ColorBuffer::desc(), TransformBuffer::desc()],
            wgpu::PrimitiveTopology::LineList
        ));
        self.pl_line_strip = Some(create_pipeline!(
            device, config, bind_group_layouts,
            shaders, layout, "Points Pipeline",
            &[PointBuffer::desc(), ColorBuffer::desc(), TransformBuffer::desc()],
            wgpu::PrimitiveTopology::LineStrip
        ));
        self.pl_tri = Some(create_pipeline!(
            device, config, bind_group_layouts,
            shaders, layout, "Points Pipeline",
            &[PointBuffer::desc(), ColorBuffer::desc(), TransformBuffer::desc()],
            wgpu::PrimitiveTopology::TriangleList
        ));
        self.pl_tri_strip = Some(create_pipeline!(
            device, config, bind_group_layouts,
            shaders, layout, "Points Pipeline",
            &[PointBuffer::desc(), ColorBuffer::desc(), TransformBuffer::desc()],
            wgpu::PrimitiveTopology::TriangleStrip
        ));
        self.device = Some(device);
        self.queue = Some(queue);
    }
    pub fn request_index(&mut self) -> usize {
        self.counter += 1;
        self.counter
    }
    pub fn add_item(&mut self, buffer: ShapeRenderer, id: Option<usize>) {
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
    pub fn entry(&self, id: &usize) -> Option<&ShapeRenderer> {
        self.map.get(id)
    }
    pub fn entry_mut(&mut self, id: &usize) -> Option<&mut ShapeRenderer> {
        self.map.get_mut(id)
    }

    fn render_pipeline(&mut self, device: &wgpu::Device, pass: &mut wgpu::RenderPass, ty: RenderType) {
        for v in self.map.values_mut() {
            if v.get_render_type() == ty {
                v.render(device, pass);
            }
        }
    }
}

macro_rules! render {
    ($op: ident, $view: ident, $encoder: ident, $clear: ident, $bind_group: ident) => {{
        let pipeline = if let Some(p) = $op.as_ref() { p } else { return; };
        let mut pass = $encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    $view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: if $clear {wgpu::LoadOp::Clear(wgpu::Color::BLACK)}
                            else {wgpu::LoadOp::Load},
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            }
        );
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, $bind_group, &[]);
        pass
    }};
}

impl RenderManager for ShapeManager {
    fn render(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        bind_group: &wgpu::BindGroup,
        _queue: &wgpu::Queue,
    ) {
        {
            let mut pass = {
                let p = self.pl_points.as_ref();
                render!(p, view, encoder, true, bind_group)
            };
            self.render_pipeline(device, &mut pass, RenderType::Points);
        }
        {
            let mut pass = {
            let p = self.pl_line.as_ref();
            render!(p, view, encoder, false, bind_group)
            };
            self.render_pipeline(device, &mut pass, RenderType::Line);
        }
        {
            let mut pass = {
            let p = self.pl_line_strip.as_ref();
            render!(p, view, encoder, false, bind_group)
            };
            self.render_pipeline(device, &mut pass, RenderType::LineStrip);
        }
        {
            let mut pass = {
            let p = self.pl_tri.as_ref();
            render!(p, view, encoder, false, bind_group)
            };
            self.render_pipeline(device, &mut pass, RenderType::Triangle);
        }
        {
            let mut pass = {
            let p = self.pl_tri_strip.as_ref();
            render!(p, view, encoder, false, bind_group)
            };
            self.render_pipeline(device, &mut pass, RenderType::TriangleStrip);
        }
    }
}