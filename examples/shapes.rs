use std::{cell::RefCell, rc::Rc, vec};

use fcv::{renders::shape_renderer::ShapeRenderer, shapes::ShapeBase, window::{FcvWindow, FcvWindowConfig}};

use glam::{Mat4, Vec3, Vec4};



fn main() {
    env_logger::init();

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

        let id = manager.request_index();
        manager.add_item(renderer, Some(id));
        let shape = manager.entry_mut(&id).unwrap();

        let m = vec![
            Mat4::from_translation(Vec3::X) * Mat4::from_rotation_x(45f32.to_radians()),
            Mat4::from_translation(Vec3::Y) * Mat4::from_rotation_y(45f32.to_radians()),
            Mat4::from_translation(Vec3::Z) * Mat4::from_rotation_z(45f32.to_radians()),
        ];

        shape.set_instances(&m);

        manager.add_item(ShapeRenderer::new(Rc::new(RefCell::new(
            ShapeBase::new_arrow(0.3, 2., 0.2, 0.8, 16, Vec4::new(1., 0., 0., 1.), false).with_transform(Mat4::from_rotation_y(90f32.to_radians()))
            .combination(&[
                ShapeBase::new_arrow(0.3, 2., 0.2, 0.8, 16, Vec4::new(0., 1., 0., 1.), false).with_transform(Mat4::from_rotation_x(-90f32.to_radians())),
                ShapeBase::new_arrow(0.3, 2., 0.2, 0.8, 16, Vec4::new(0., 0., 1., 1.), false),
                ShapeBase::new_sphere(0.1, 16, 16, Vec4::ONE, false)
            ])
        ))) , None);
    }

    window.render_loop(
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