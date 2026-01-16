use states::State;
use ui::menu::Button;

use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{WindowEvent, MouseButton};
use winit::event::KeyEvent;

pub struct PauseSubState {
    background_vec: Vec<f32>,
    buttons: Vec<Button>,
    mouse_position: PhysicalPosition<f64>, 
    state: &State
}

impl PauseSubState {
    pub fn new(background_vec: Vec<f32>, mouse_position: PhysicalPosition<f64>, state: &State) -> Box<Self> {
        let buttons = [
                   Button::new([0.4, 0.1], 0.2, 0.1, "", 0),
                   Button::new([0.4, 0.3], 0.2, 0.1, "", 1), 
                   Button::new([0.4, 0.5], 0.2, 0.1, "", 2)
                  ];
        
        Box::new(Self{background_vec, buttons, mouse_position, state})
    }
}

impl State for PauseSubState {
    fn get_vertices (&self) -> &Vec<f32> {

    }

    fn get_textures (&mut self) -> (bool, Option<&mut Vec<*mut Texture>>, Option<& Vec<f32>>, Option<& Vec<u32>>) {

    }

    fn get_text(&mut self) -> (bool, Option<&mut Vec<Text>>, Option<&mut TextFabric>) {

    }

    fn update(&mut self) {
        
    }

    fn manage_keyboard_input(&mut self, event: KeyEvent) {

    }

    fn manage_mouse_input(&mut self, button: MouseButton) {
        for button in self.buttons {
            if (button.verify_inside(self.mouse_position)) {
                match button.id {
                    0 => {
                        self.state.close_substate();       
                    },

                    1 => {
                        
                    },

                    2 => {
                            
                    }
                }
            }
        }
    }

    fn manage_mouse_movement(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_position = position;
    }

    fn manage_window_resize(&mut self, size: PhysicalSize<u32>) {
        
    }

    fn close_substate(&mut self) {

    }
}
