pub mod vertex_renders;
pub mod sparse_vertex_renders;
pub mod lines_renders;
pub mod faces_renders;

pub mod vertex_manager;

#[derive(Debug, Clone, Copy)]
pub enum BufferUsageType {
    Points,
    Colors,
    PointsAndColors,
    Indices,
}

#[macro_export]
macro_rules! create_pipeline {
    (
        $device: ident,
        $config: ident,
        $bindings: ident,
        $shader: ident,
        $layout: ident,
        $pipeline_label: expr,
        $buffers: expr
    ) => {
        $device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some($pipeline_label),
            layout: Some(&$layout),
            vertex: wgpu::VertexState {
                module: &$shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: $buffers,
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &$shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: $config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        })
    };
}