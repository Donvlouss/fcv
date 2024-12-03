
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};
use std::cell::RefMut;
use egui_wgpu::wgpu::{self, include_wgsl};
use glam::{Vec3, Vec4};

use crate::{buffers::{transform_buffer::TransformBuffer, vertex_buffer::{ColorBuffer, PointBuffer}, BufferType}, create_pipeline, shapes::{RenderShape, RenderType, ShapeBase}, texture::FcvTexture};

use super::{shape_renderer::ShapeRenderer, RenderManager};

#[derive(Default)]
pub struct ShapeManager {
    map: HashMap<usize, ShapeRenderer>,
    single_map: HashMap<RenderType, Rc<RefCell<ShapeBase>>>,
    counter: usize,

    pipelines: HashMap<RenderType, wgpu::RenderPipeline>,

    device: Option<Arc<wgpu::Device>>,
    queue: Option<Arc<wgpu::Queue>>,
}

impl RenderType {
    const PIPELINE_LABELS: [&'static str; 7] = [
        "Points Pipeline",
        "Line Pipeline",
        "LineStrip Pipeline",
        "Triangle Pipeline",
        "TriangleStrip Pipeline",
        "Transparent Triangle Pipeline",
        "Transparent TriangleStrip Pipeline",
    ];
    pub fn pipeline_label(&self) -> &'static str {
        Self::PIPELINE_LABELS[(*self as u8) as usize]
    }
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
                label: Some("Pipelines Layout"),
                bind_group_layouts,
                push_constant_ranges: &[],
            }
        );
        let mut map = HashMap::new();
        for t in RenderType::iter() {
            let transparent = if *t == RenderType::TriangleTransparent || *t == RenderType::TriangleStripTransparent { true } else { false }; 
            map.insert(*t, create_pipeline!(
                device, config, bind_group_layouts,
                shaders, layout, t.pipeline_label(),
                &[PointBuffer::desc(), ColorBuffer::desc(), TransformBuffer::desc()],
                t.wgpu_raw(),
                transparent
            ));
        }
        self.pipelines = map;

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

    fn render_partial(&mut self, device: &wgpu::Device, pass: &mut wgpu::RenderPass, ids: &Vec<usize>, ty: RenderType) {
        for id in ids {
            self.map.get_mut(id).expect("Get RenderShape from ids.")
                .render(device, pass);
        }
        if let Some(shape) = self.single_map.get(&ty) {
            ShapeRenderer::new(Rc::clone(shape) as Rc<RefCell<dyn RenderShape>>).render(device, pass);
        }
    }
    fn single_entry_mut(&mut self, ty: RenderType) -> RefMut<ShapeBase> {
        self.single_map.entry(ty).or_insert(Rc::new(RefCell::new(ShapeBase::default().set_type(ty)))).borrow_mut()
    }

    pub fn draw_point(&mut self, pt: Vec3, color: Vec4) {
        let mut entry = self.single_entry_mut(RenderType::Points);
        let n = entry.points.len() as u32;
        entry.indices.push(n);
        entry.points.push(pt);
        entry.colors.push(color);
    }
    pub fn draw_line(&mut self, a: Vec3, b: Vec3, color: Vec4) {
        let mut entry = self.single_entry_mut(RenderType::Line);
        let n = entry.points.len() as u32;
        entry.indices.push(n);
        entry.indices.push(n+1);
        entry.points.push(a);
        entry.points.push(b);
        entry.colors.push(color);
    }
    pub fn draw_triangle(&mut self, pts: &[Vec3], color: &[Vec4]) {
        let mut entry = self.single_entry_mut(RenderType::Triangle);
        let mut n = entry.points.len() as u32;
        for p in pts.iter() {
            entry.points.push(*p);
            entry.indices.push(n);
            n+=1;
        }
        for c in color.iter() {
            entry.colors.push(*c);
        }
    }
    fn prepare(&self) -> Vec<Vec<usize>> {
        let mut list = vec![vec![]; RenderType::len()];
        self.map.iter()
            .for_each(|(id, s)| {
                let t = if s.transparent() {
                    if s.get_render_type() == RenderType::Triangle { RenderType::TriangleTransparent } else { RenderType::TriangleStripTransparent }
                } else { s.get_render_type() };
                list[(t as u8) as usize].push(*id);
            });
        list
    }
    pub fn clear_single(&mut self) {
        self.single_map.clear();
    }
}

macro_rules! render {
    ($pipeline: ident, $label: expr, $view: ident, $encoder: ident, $clear: ident, $bind_group: ident, $depth_view: ident, $transparent: ident) => {{
        let mut pass = $encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some($label),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    $view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: if $clear {wgpu::LoadOp::Clear(wgpu::Color::BLACK)}
                            else {wgpu::LoadOp::Load},
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: if $transparent { None } else { Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: $depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }
                ) },
                timestamp_writes: None,
                occlusion_query_set: None,
            }
        );
        pass.set_pipeline($pipeline);
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
        depth_view: &wgpu::TextureView,
        _queue: &wgpu::Queue,
    ) {
        let render_list = self.prepare();
        let mut first = true;
        for (k, v) in RenderType::iter().zip(&render_list) {
            let p = self.pipelines.get(k).unwrap();
            let transparent = *k == RenderType::TriangleTransparent || *k == RenderType::TriangleStripTransparent;
            let mut pass = render!(p, k.pipeline_label(), view, encoder, first, bind_group, depth_view, transparent);
            first = false;
            self.render_partial(device, &mut pass, v, *k);
        }
    }
}

impl ShapeManager {
    pub fn add_square(
        &mut self,
        size: f32,
        color: Vec4,
        wire: bool,
    ) {
        self.add_item(ShapeRenderer::new(
            Rc::new(RefCell::new(
                ShapeBase::new_square(size, color, wire)
            ))
        ), None);
    }
    pub fn add_cube(
        &mut self,
        size: f32,
        face_color: [Vec4; 6],
        wire: bool,
    ) {
        self.add_item(ShapeRenderer::new(
            Rc::new(RefCell::new(
                ShapeBase::new_cube(size, face_color, wire)
            ))
        ), None);
    }
    pub fn add_circle(
        &mut self,
        r: f32,
        sub: u32,
        color: Vec4,
        wire: bool,
    ) {
        self.add_item(ShapeRenderer::new(
            Rc::new(RefCell::new(
                ShapeBase::new_circle(r, sub, color, wire)
            ))
        ), None);
    }
    pub fn add_sphere(
        &mut self,
        r: f32,
        u_sub: u32,
        v_sub: u32,
        color: Vec4,
        wire: bool
    ) {
        self.add_item(ShapeRenderer::new(
            Rc::new(RefCell::new(
                ShapeBase::new_sphere(r, u_sub, v_sub, color, wire)
            ))
        ), None);
    }
    pub fn add_cone(
        &mut self,
        r: f32,
        u_sub: u32,
        height: f32,
        color: Vec4,
        wire: bool
    ) {
        self.add_item(ShapeRenderer::new(
            Rc::new(RefCell::new(
                ShapeBase::new_cone(r, u_sub, height, color, wire)
            ))
        ), None);
    }
    pub fn add_cylinder(
        &mut self,
        r: f32,
        u_sub: u32,
        height: f32,
        color: Vec4,
        wire: bool
    ) {
        self.add_item(ShapeRenderer::new(
            Rc::new(RefCell::new(
                ShapeBase::new_cylinder(r, u_sub, height, color, wire)
            ))
        ), None);
    }
    pub fn add_arrow(
        &mut self,
        arrow_radius: f32,
        height: f32,
        tail_ratio: f32,
        tail_height_ratio: f32,
        u_sub: u32,
        color: Vec4,
        wire: bool,
    ) {
        self.add_item(ShapeRenderer::new(
            Rc::new(RefCell::new(
                ShapeBase::new_arrow(arrow_radius, height, tail_ratio, tail_height_ratio, u_sub, color, wire)
            ))
        ), None);
    }
}