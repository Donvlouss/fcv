use egui_wgpu::wgpu;

pub mod shape_manager;
pub mod shape_renderer;

#[derive(Debug, Clone, Copy)]
pub enum BufferUsageType {
    Points,
    Colors,
    PointsAndColors,
    Indices,
}

pub trait RenderManager {
    fn render(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        bind_group: &wgpu::BindGroup,
        queue: &wgpu::Queue,
    );
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
        $buffers: expr,
        $topology: expr
    ) => {
        $device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some($pipeline_label),
            layout: Some(&$layout),
            vertex: wgpu::VertexState {
                module: &$shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: $buffers,
            },
            primitive: wgpu::PrimitiveState {
                topology: $topology,
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
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: $config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        })
    };
}