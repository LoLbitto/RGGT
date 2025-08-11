use crate::ui::menu::button::Button;
use crate::app::App;
use crate::states::State;
use crate::states::play_state::PlayState;

use image::ImageReader;

use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{WindowEvent, MouseButton};
use winit::event::KeyEvent;

const DEFAULT_Z: f32 = 0.0;
const DEFAULT_W: f32 = 1.0;

const PLAY_BUTTON: i32 = 1;
const EXIT_BUTTON: i32 = 2;

pub struct MainMenuState {
    buttons: Vec<Button>,
    mouse_position: PhysicalPosition<f64>,
    vertices: Vec<f32>,
    app: *mut App
}

impl MainMenuState {
    pub fn new(app: &mut App) -> Box<Self> {
        let cords_play = [-0.8, 0.0];
        let cords_exit = [-0.8, -0.25];

        let width = 0.3;
        let height = 0.2;

        let button_play = Button::new(cords_play, width, height, PLAY_BUTTON as u32);
        let button_exit = Button::new(cords_exit, width, height, EXIT_BUTTON as u32);

        let buttons = vec![button_play, button_exit];

        let mut vertices = Vec::<f32>::new();

        for i in 0..buttons.len() {
            let (red, green, blue) = (i as f32, 0.0, 1.0);

            for j in 0..buttons[i].points.len() / 2 - 1{
                let index = j * 2;
                let (x, y) = (buttons[i].points[index], buttons[i].points[index+1]);

                vertices.push(x);
                vertices.push(y);
                vertices.push(DEFAULT_Z);
                vertices.push(DEFAULT_W);
                vertices.push(red);
                vertices.push(green);
                vertices.push(blue);
            } // Primeiro triângulo

            for j in 1..buttons[i].points.len() / 2{
                let index = j * 2;
                let (x, y) = (buttons[i].points[index], buttons[i].points[index+1]);

                vertices.push(x);
                vertices.push(y);
                vertices.push(DEFAULT_Z);
                vertices.push(DEFAULT_W);
                vertices.push(red);
                vertices.push(green);
                vertices.push(blue);
            } // Segundo triângulo
        }       

        let mouse_position = PhysicalPosition::<f64>::new(0.0, 0.0); 

        Box::new(Self {buttons, mouse_position, vertices, app})
    }
}

impl State for MainMenuState {
    fn get_vertices (&self) -> &Vec<f32> {
        &self.vertices
    }

    fn update(&mut self) {
        
    }

    fn manage_keyboard_input(&mut self, event: KeyEvent) {

    }

    fn manage_mouse_input(&mut self, button: MouseButton) {
        let mut id = -1;
        let position = self.mouse_position;
        
        if (button == MouseButton::Left) {   
            for button in &self.buttons {
                let check = button.verify_inside(position);

                if check {
                    id = button.id as i32;
                    break;
                }
            }
        }

        match id {
            PLAY_BUTTON => {
                unsafe{
                    let state = PlayState::new("teste".to_string(), &mut *self.app) as Box<dyn State>;
                    (*self.app).game_state.replace(state);
                }
            },

            _ => {}
        }        
    }

    fn manage_mouse_movement(&mut self, position: PhysicalPosition<f64>) {
        self.mouse_position = position;
    }

    fn manage_window_resize(&mut self, size: PhysicalSize<u32>) {
        for i in 0..self.buttons.len() {
            self.buttons[i].update_screen_size(size);
        }
    }
}
