use std::{cell::RefCell, rc::Rc};

use fcv::{renders::shape_renderer::ShapeRenderer, shapes::ShapeBase, window::{FcvWindow, FcvWindowConfig}};

use glam::{Vec3, Vec4};
use winit::{event_loop::EventLoop, platform::windows::EventLoopBuilderExtWindows};



fn main() {
    env_logger::init();
    let event_loop = EventLoop::builder()
        .with_any_thread(true)
        .build()
        .unwrap();

    let mut window = FcvWindow::new(
        FcvWindowConfig {
            title: "Simple Visualization".to_owned(),
            mode: fcv::window::WindowUpdateMode::StaticTime(1. / 144.),
            ..Default::default()
        }
    );

    {
        let manager = window.manager();
        // let renderer = ShapeRenderer::new(
        //     Rc::new(RefCell::new(ShapeBase::new_cube(1., [Vec4::ONE; 6], false)))
        // );
        // let renderer = ShapeRenderer::new(
        //     Rc::new(RefCell::new(ShapeBase::new_square(1., Vec4::ONE, false)))
        // );
        let renderer = ShapeRenderer::new(
            Rc::new(RefCell::new(
                ShapeBase::new_raw(
                    fcv::shapes::InitType::Move(vec![Vec3::ZERO, Vec3::X]),
                    fcv::shapes::ColorType::Uniform(Vec4::ONE),
                    fcv::shapes::IndicesType::Sequence,
                    fcv::shapes::RenderType::Line
                )
            ))
        );
        manager.add_item(renderer, None);
    }

    window.render_loop(
        event_loop,
        |ctx, _vertex_manager| {
            egui::Window::new("winit + egui + wgpu says hello!")
                .resizable(true)
                .vscroll(true)
                .default_open(false)
                .show(ctx, |ui| {
                    ui.label("Label!");

                    if ui.button("Button!").clicked() {
                        println!("boom!")
                    }

                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "Pixels per point: {}",
                            ctx.pixels_per_point()
                        ));
                    });
                });
        },
    );
}