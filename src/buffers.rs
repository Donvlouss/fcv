pub mod vertex_buffer;
pub mod vertex_impl;

pub mod transform_buffer;


pub trait BufferType
where Self: Sized
{
    fn desc<'ds>() -> egui_wgpu::wgpu::VertexBufferLayout<'ds>;
    fn size() -> u64 {
        std::mem::size_of::<Self>() as u64
    }
}