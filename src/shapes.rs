pub mod shape_base;

use crate::buffers::vertex_buffer::{ColorBuffer, PointBuffer};
use glam::{Mat4, Vec3, Vec4};

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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RenderType {
    #[default]
    Points,
    Line,
    LineStrip,
    Triangle,
    TriangleStrip
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
    fn shape_type(&self) -> ShapeType;
    // Render
    fn should_repaint(&self) -> bool;
    fn get_render_type(&self) -> RenderType;
    // Buffer cache
    fn set_index(&mut self, index: usize);
    fn get_index(&self) -> usize;
    // Build buffer
    fn points(&self) -> &[PointBuffer];
    fn colors(&self) -> &[ColorBuffer];
    fn indices(&self) -> &[u32];
    // Transform
    fn transform(&self) -> [[f32; 4]; 4];
    // Parent
    fn parent(&self) -> Option<usize>;
}

#[derive(Debug, Clone, Default)]
pub struct ShapeBase {
    shape: ShapeType,

    modified: bool,
    render_type: RenderType,

    id: usize,

    points: Vec<Vec3>,
    colors: Vec<Vec4>,
    indices: Vec<u32>,

    transform: Mat4,

    parent: Option<usize>,
}
