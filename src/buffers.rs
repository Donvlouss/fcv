pub mod vertex_buffer;
pub mod vertex_impl;
pub trait BufferType
where Self: Sized
{
    fn desc<'ds>() -> wgpu::VertexBufferLayout<'ds>;
    fn size() -> u64 {
        std::mem::size_of::<Self>() as u64
    }
}