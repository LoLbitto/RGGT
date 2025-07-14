mod app;
mod renderer;
mod object;

use crate::app::App;
use crate::app::window_attributes;

use glutin_winit::DisplayBuilder;
use winit::event_loop::EventLoop;
use winit::event_loop::ControlFlow;
use winit::window::Window;

use glutin::config::ConfigTemplateBuilder;

fn main() {
    
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.set_control_flow(ControlFlow::Wait);

    let template = ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(cfg!(cgl_backend));

    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes()));

    let mut app = App::new(template, display_builder);
    event_loop.run_app(&mut app);
}
