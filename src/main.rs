//#![windows_subsystem = "windows"]

mod app;
mod renderer;
mod graphic;
mod entity;
mod resources;

use std::sync::Mutex;

use crate::app::App;
use crate::app::window_attributes;
use crate::resources::file_manager;

use glutin_winit::DisplayBuilder;
use winit::event_loop::EventLoop;
use winit::event_loop::ControlFlow;

use glutin::config::ConfigTemplateBuilder;

fn main() {

    let template = ConfigTemplateBuilder::new().with_alpha_size(8).with_transparency(cfg!(cgl_backend));

    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes()));

    let mut app = App::new(template, display_builder); 

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut app);

    file_manager::get_object("piramide".to_string());
}
