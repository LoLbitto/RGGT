use crate::ui::menu::button::Button;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{WindowEvent, MouseButton};

const PLAY_BUTTON: u32 = 1;
const EXIT_BUTTON: u32 = 2;

struct MainMenuState {
    buttons: Vec<Button>,
    mouse_position: PhysicalPosition<u32>,
}

impl MainMenuState {
    pub fn new() -> Self {
        let cords_play = [-0.8, 0.0];
        let cords_exit = [-0.8, -0.25];

        let width = 0.3;
        let height = 0.2;

        let button_play = Button::new(cords_play, width, height, PLAY_BUTTON);
        let button_exit = Button::new(cords_exit, width, height, EXIT_BUTTON);

        let mut buttons = vec![button_play, button_exit];

        let mouse_position = PhysicalPosition::<u32>::new(0, 0); 

        Self {buttons, mouse_position}
    }
}

impl State for MainMenuState {
    fn get_vertices (&self) -> &Vec<f32> {
        
    }

    fn update(&mut self) {
        
    }

    fn manage_keyboard_input(&mut self, event: KeyEvent) {

    }

    fn manage_mouse_input(&mut self, event: WindowEvent) {
        let mut id = -1;
        let position = self.mouse_position;
        
        if (event.button == MouseButton::Left) {
            for button in self.buttons {
                let check = button.verify_inside(position);

                if check {
                    id = button.id; 
                }
            }
        }
    }

    fn manage_mouse_movement(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_position = position;
    }

    fn manage_window_resize(&mut self, size: PhysicalSize<u32>) {
        for button in self.buttons {
            button.update_screen_size(size);
        }
    }
}
