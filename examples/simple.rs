use fcv::window::{FcvWindow, FcvWindowConfig};

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

    let x_axis = (0..=100)
        .map(|i| Vec3::new(i as f32 / 100., 0., 0.)).collect::<Vec<_>>();
    let y_axis = (0..=100)
        .map(|i| Vec3::new(0., i as f32 / 100., 0.)).collect::<Vec<_>>();
    let z_axis = (0..=100)
        .map(|i| Vec3::new(0., 0., i as f32 / 100.)).collect::<Vec<_>>();
    window.add_points_uniform_color(&x_axis, Vec4::new(1., 0., 0., 1.,));
    window.add_points_uniform_color(&y_axis, Vec4::new(0., 1., 0., 1.,));
    window.add_points_uniform_color(&z_axis, Vec4::new(0., 0., 1., 1.,));

    let line_pts = (0..=100)
        .map(|i| {
            let p = i as f32 / 100.;
            Vec3::new(p, p, 0.)
        }).collect::<Vec<_>>();
    let white = Vec4::ONE;

    window.render_loop(
        event_loop,
        move |vertex_manager| {
            line_pts.iter().for_each(|p| {
                vertex_manager.draw_point(*p, white);
            });
        }
    );
}