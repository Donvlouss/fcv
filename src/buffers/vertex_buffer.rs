use super::BufferType;

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct PointBuffer {
    pub point: [f32; 3],
}
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ColorBuffer {
    pub color: [f32; 4],
}
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct PointColorBuffer {
    pub point: [f32; 3],
    pub color: [f32; 4],
}

impl BufferType for PointBuffer {
    fn desc<'ds>() -> wgpu::VertexBufferLayout<'ds> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<PointBuffer>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
            ],
        }
    }
}
impl BufferType for ColorBuffer {
    fn desc<'ds>() -> wgpu::VertexBufferLayout<'ds> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ColorBuffer>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 1,
                },
            ],
        }
    }
}
impl BufferType for PointColorBuffer {
    fn desc<'ds>() -> wgpu::VertexBufferLayout<'ds> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<PointColorBuffer>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 1,
                },
            ],
        }
    }
}