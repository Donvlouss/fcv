use glam::{Vec3, Vec4};

use crate::renders::vertex_renders::VertexBuffer;

use super::FcvContext;

impl<'window> FcvContext<'window> {
    pub fn add_points_with_indices(
        &mut self,
        points: &[Vec3],
        colors: &[Vec4],
        indices: &[usize]
    ) -> usize {
        let id = self.vertex_manager.request_index();
        let buffer = VertexBuffer::new(id)
            .set_points(&self.device, &self.queue,
                bytemuck::cast_slice(points))
            .set_colors(&self.device, &self.queue,
                bytemuck::cast_slice(colors))
            .set_indices(&self.device, &self.queue,
                bytemuck::cast_slice(indices));
        self.vertex_manager.add_item(buffer, Some(id));
        id
    }
    pub fn add_points(
        &mut self,
        points: &[Vec3],
        colors: &[Vec4],
    ) -> usize {
        let indices = (0..points.len().min(colors.len())).collect::<Vec<_>>();
        self.add_points_with_indices(points, colors, &indices)
    }
    pub fn add_points_uniform_color(
        &mut self,
        points: &[Vec3],
        color: Vec4,
    ) -> usize {
        let colors = vec![color; points.len()];
        self.add_points(points, &colors)
    }
    pub fn remove_points(&mut self, id: usize) {
        self.vertex_manager.remove_item(id);
    }
    pub fn draw_point(&mut self, p: Vec3, c: Vec4) {
        self.vertex_manager.draw_point(p, c);
    }
}