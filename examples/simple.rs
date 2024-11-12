use fcv::window::{FcvWindow, FcvWindowConfig};

use winit::event_loop::EventLoop;



fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    
    let mut window = FcvWindow::new(
        FcvWindowConfig {
            title: "Simple Visualization".to_owned(),
        }
    );


    event_loop.run_app(&mut window).unwrap()
}