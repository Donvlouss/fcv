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
            mode: fcv::window::WindowUpdateMode::StaticTime(1. / 60.),
            ..Default::default()
        }
    );

    {
        let manager = window.manager();
        // Cube
        let renderer = ShapeRenderer::new(
            Rc::new(RefCell::new(ShapeBase::new_cube(1., [
                Vec4::new(1., 0., 0., 0.5), Vec4::new(1., 0., 0., 0.5),
                Vec4::new(0., 1., 0., 0.5), Vec4::new(0., 1., 0., 0.5),
                Vec4::new(0., 0., 1., 0.5), Vec4::new(0., 0., 1., 0.5),
            ], false)))
        );

        // Square
        // let renderer = ShapeRenderer::new(
        //     Rc::new(RefCell::new(ShapeBase::new_square(1., Vec4::ONE, true)))
        // );
        
        // Circle
        // let renderer = ShapeRenderer::new(
        //     Rc::new(RefCell::new(ShapeBase::new_circle(1., 16, Vec4::ONE, false)))
        // );

        // Sphere (cylinder ?)
        // let renderer = ShapeRenderer::new(
        //     Rc::new(RefCell::new(ShapeBase::new_sphere(1., 16, 16, Vec4::ONE, false)))
        // );

        // Cone
        // let renderer = ShapeRenderer::new(
        //     Rc::new(RefCell::new(ShapeBase::new_cone(1., 4, 1., Vec4::ONE, true)))
        // );

        // Cylinder
        // let renderer = ShapeRenderer::new(
        //     Rc::new(RefCell::new(ShapeBase::new_cylinder(0.5, 16, 1., Vec4::ONE, true)))
        // );

        // Arrow
        // let renderer = ShapeRenderer::new(
        //     Rc::new(RefCell::new(ShapeBase::new_arrow(0.2, 1., 0.5, 0.8, 16, Vec4::ONE, false)))
        // );


        manager.add_item(renderer, None);

        // Axis
        manager.add_item(ShapeRenderer::new(Rc::new(RefCell::new(
            ShapeBase::new_raw(
                fcv::shapes::InitType::Move(vec![
                        Vec3::ZERO, Vec3::X * 2.,
                        Vec3::ZERO, Vec3::Y * 2.,
                        Vec3::ZERO, Vec3::Z * 2.,
                    ]
                ),
                fcv::shapes::ColorType::Each(fcv::shapes::InitType::Move(vec![
                        Vec4::new(1., 0., 0., 1.), Vec4::new(1., 0., 0., 1.),
                        Vec4::new(0., 1., 0., 1.), Vec4::new(0., 1., 0., 1.),
                        Vec4::new(0., 0., 1., 1.), Vec4::new(0., 0., 1., 1.),
                    ]
                )),
            fcv::shapes::IndicesType::Partial(
                fcv::shapes::InitType::Move(vec![0, 1, 2, 3, 4, 5])
            ), fcv::shapes::RenderType::Line)
            ))),
            None
        );
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