use glam::{Vec2, Vec3, Vec3A, Vec4};

use super::vertex_buffer::{ColorBuffer, PointBuffer, PointColorBuffer};

pub trait ToPointBuffer {
    fn convert_pb(&self) -> PointBuffer;
}
pub trait ToColorBuffer {
    fn convert_cb(&self) -> ColorBuffer;
}
pub trait ToPointColorBuffer {
    fn convert_pcb(&self) -> PointColorBuffer;
}

impl ToPointBuffer for Vec2 {
    fn convert_pb(&self) -> PointBuffer {
        PointBuffer { point: [self.x, self.y, 0.] }
    }
}
impl ToPointBuffer for Vec3 {
    fn convert_pb(&self) -> PointBuffer {
        PointBuffer { point: [self.x, self.y, self.z] }
    }
}
impl ToPointBuffer for Vec3A {
    fn convert_pb(&self) -> PointBuffer {
        PointBuffer { point: [self.x, self.y, self.z] }
    }
}
impl ToPointBuffer for Vec4 {
    fn convert_pb(&self) -> PointBuffer {
        PointBuffer { point: [self.x, self.y, self.z] }
    }
}
impl ToColorBuffer for Vec2 {
    fn convert_cb(&self) -> ColorBuffer {
        ColorBuffer { color: [self.x, self.y, 0., 1.] }
    }
}
impl ToColorBuffer for Vec3 {
    fn convert_cb(&self) -> ColorBuffer {
        ColorBuffer { color: [self.x, self.y, self.z, 1.] }
    }
}
impl ToColorBuffer for Vec3A {
    fn convert_cb(&self) -> ColorBuffer {
        ColorBuffer { color: [self.x, self.y, self.z, 1.] }
    }
}
impl ToColorBuffer for Vec4 {
    fn convert_cb(&self) -> ColorBuffer {
        ColorBuffer { color: [self.x, self.y, self.z, self.w] }
    }
}
impl From<Vec2> for PointBuffer {
    fn from(value: Vec2) -> Self {
        Self { point: [value.x, value.y, 0.] }
    }
}
impl From<Vec3> for PointBuffer {
    fn from(value: Vec3) -> Self {
        Self { point: [value.x, value.y, value.z] }
    }
}
impl From<Vec3A> for PointBuffer {
    fn from(value: Vec3A) -> Self {
        Self { point: [value.x, value.y, value.z] }
    }
}
impl From<Vec4> for PointBuffer {
    fn from(value: Vec4) -> Self {
        Self { point: [value.x, value.y, value.z] }
    }
}
impl From<Vec2> for ColorBuffer {
    fn from(value: Vec2) -> Self {
        Self { color: [value.x, value.y, 0., 1.] }
    }
}
impl From<Vec3> for ColorBuffer {
    fn from(value: Vec3) -> Self {
        Self { color: [value.x, value.y, value.z, 1.] }
    }
}
impl From<Vec3A> for ColorBuffer {
    fn from(value: Vec3A) -> Self {
        Self { color: [value.x, value.y, value.z, 1.] }
    }
}
impl From<Vec4> for ColorBuffer {
    fn from(value: Vec4) -> Self {
        Self { color: [value.x, value.y, value.z, value.w] }
    }
}