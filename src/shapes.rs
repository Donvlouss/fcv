pub mod shape_base;

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
    // Render
    fn should_repaint(&self) -> bool;
    fn set_repaint(&mut self, repaint: bool);
    fn get_render_type(&self) -> RenderType;
    // Build buffer
    fn points(&self) -> &[u8];
    fn colors(&self) -> &[u8];
    fn indices(&self) -> &[u32];
}

#[derive(Debug, Clone, Default)]
pub struct ShapeBase {
    pub modified: bool,
    pub render_type: RenderType,

    pub points: Vec<Vec3>,
    pub colors: Vec<Vec4>,
    pub indices: Vec<u32>,
}
