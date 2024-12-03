pub mod shape_base;

use std::slice::Iter;
use egui_wgpu::wgpu;
use glam::{Vec3, Vec4};

#[derive(Debug, Clone, Copy, Default)]
pub enum ShapeType {
    #[default]
    Polygon,
    Square,
    Cube,
    Circle,
    Sphere,
    Cone,
    Cylinder,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RenderType {
    #[default]
    Points,
    Line,
    LineStrip,
    Triangle,
    TriangleStrip,
    TriangleTransparent,
    TriangleStripTransparent,
}

impl RenderType {
    pub fn len() -> usize { 7 }

    pub fn iter() -> Iter<'static, RenderType> {
        static ITERS: [RenderType; 7] = [
            RenderType::Points,RenderType::Line, RenderType::LineStrip, RenderType::Triangle, RenderType::TriangleStrip,
            RenderType::TriangleTransparent, RenderType::TriangleStripTransparent,
        ];
        ITERS.iter()
    }
    
    pub fn wgpu_raw(self) -> wgpu::PrimitiveTopology { 
        match self {
            RenderType::Points => wgpu::PrimitiveTopology::PointList,
            RenderType::Line => wgpu::PrimitiveTopology::LineList,
            RenderType::LineStrip => wgpu::PrimitiveTopology::LineStrip,
            RenderType::Triangle => wgpu::PrimitiveTopology::TriangleList,  
            RenderType::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
            RenderType::TriangleTransparent => wgpu::PrimitiveTopology::TriangleList,
            RenderType::TriangleStripTransparent => wgpu::PrimitiveTopology::TriangleStrip,
        }
    }

    pub fn from_id(id: usize) -> Self {
        match id {
            0 => Self::Points,
            1 => Self::Line,
            2 => Self::LineStrip,
            3 => Self::Triangle,
            4 => Self::TriangleStrip,
            5 => Self::TriangleTransparent,
            6 => Self::TriangleStripTransparent,
            _ => Self::Points
        }
    }
}

pub enum InitType<'i, T> {
    Ref(&'i [T]),
    Move(Vec<T>),
}

pub enum ColorType<'i> {
    Each(InitType<'i, Vec4>),
    Uniform(Vec4),
}

pub enum IndicesType<'i> {
    Sequence,
    Partial(InitType<'i, u32>)
}

pub trait RenderShape {
    // Render
    fn should_repaint(&self) -> bool;
    fn set_repaint(&mut self, repaint: bool);
    fn get_render_type(&self) -> RenderType;
    // Build buffer
    fn points(&self) -> &[u8];
    fn colors(&self) -> &[u8];
    fn indices(&self) -> &[u32];
    fn transparent(&self) -> bool;
}

#[derive(Debug, Clone, Default)]
pub struct ShapeBase {
    pub modified: bool,
    pub render_type: RenderType,

    pub points: Vec<Vec3>,
    pub colors: Vec<Vec4>,
    pub indices: Vec<u32>,
}
