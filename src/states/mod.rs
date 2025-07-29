pub mod play_state;

use winit::event::{WindowEvent, KeyEvent};
use winit::window::Window;
use winit::dpi::{PhysicalPosition, PhysicalSize};


use crate::logical::entity::object::Object;
use crate::app::AppState;

pub trait State {
    
    fn get_vertices (&self) -> &Vec<f32>;
    fn update(&mut self);
    fn manage_keyboard_input(&mut self, event: KeyEvent);
    fn manage_mouse_input(&mut self, event: WindowEvent);
    fn manage_mouse_movement(&mut self, position: PhysicalPosition<f64>);
    fn manage_window_resize(&mut self, size: PhysicalSize<u32>);

    // fn set_app_state(&mut self, appstate: AppState);   
}
