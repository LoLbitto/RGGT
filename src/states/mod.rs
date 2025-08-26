pub mod play_state;
pub mod main_menu_state;
pub mod map_selector_state;

use winit::event::{WindowEvent, KeyEvent};
use winit::window::Window;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::MouseButton;

use crate::logical::entity::object::Object;
use crate::app::AppState;
use crate::graphic::texture::Texture;
use crate::ui::text::Text;

pub trait State {
    
    fn get_vertices (&self) -> &Vec<f32>;
    fn get_textures (&mut self) -> (bool, Option<&mut Vec<*mut Texture>>, Option<& Vec<f32>>, Option<& Vec<u32>>);
    fn get_text(&mut self) -> (bool, Option<&mut Vec<Text>>);

    fn update(&mut self);
    fn manage_keyboard_input(&mut self, event: KeyEvent);
    fn manage_mouse_input(&mut self, button: MouseButton);
    fn manage_mouse_movement(&mut self, position: PhysicalPosition<f64>);
    fn manage_window_resize(&mut self, size: PhysicalSize<u32>);

    // fn set_app_state(&mut self, appstate: AppState);   
}
