use crate::ui::menu::button::Button;
use crate::ui::text::Text;

use crate::app::App;
use crate::states::State;
use crate::states::play_state::PlayState;
use crate::states::map_selector_state::MapSelectorState;
use crate::resources::file_manager::assets;
use crate::graphic::texture::Texture;

use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{WindowEvent, MouseButton};
use winit::event::KeyEvent;

const PLAY_BUTTON: i32 = 1;
const EXIT_BUTTON: i32 = 2;

pub struct MainMenuState {
    buttons: Vec<Button>,
    mouse_position: PhysicalPosition<f64>,
    vertices: Vec<f32>,
    vertices_textura: Vec<f32>, // 2 vetores de vertices pq podem ter ambos, solido e com textura
    texturas: Vec<*mut Texture>,
    tex_map: Vec<u32>,
    app: *mut App
}

impl MainMenuState {
    pub fn new(app: &mut App) -> Box<Self> {
        let cords_play = [-0.8, 0.0];
        let cords_exit = [-0.8, -0.25];

        let width = 0.3;
        let height = 0.2;

        // NOTE: fazer assim deixa muito estático, o certo é ler de algum lugar
            //  Concordo plenamente
        let button_play = Button::new(cords_play, width, height, "play", PLAY_BUTTON as u32);
        let button_exit = Button::new(cords_exit, width, height, "exit", EXIT_BUTTON as u32);

        let mut buttons = vec![button_play, button_exit];

        let mut vertices = Vec::<f32>::new();
        let mut vertices_textura = Vec::<f32>::new();
        let mut texturas = Vec::<*mut Texture>::new();
        let mut tex_map = Vec::<u32>::new();
        // NOTE: Como não estou considerando que o idiota do usuário pode DELETAR os jpg dos
        // botões, estou considerando que nunca vai dar erro no loading da imagem, MAS isso deve
        // ser mudado futuramente. 

        for i in 0..buttons.len() {
            let but_vertices = buttons[i].get_vertices();
            
            for j in 0..but_vertices.len() {
                vertices_textura.push(but_vertices[j]);
            }

            texturas.push(buttons[i].get_texture());
            tex_map.push(i as u32);
            tex_map.push(i as u32);
        }

        let mouse_position = PhysicalPosition::<f64>::new(0.0, 0.0); 

        Box::new(Self {buttons, mouse_position, vertices, vertices_textura, texturas, tex_map, app})
    }
}

impl<'a> State<'a> for MainMenuState {
    fn get_vertices (&self) -> &Vec<f32> {
        &self.vertices
    }

    fn get_textures(&mut self) -> (bool, Option<&mut Vec<*mut Texture>>, Option<& Vec<f32>>, Option<& Vec<u32>>) {
        (true, Some(&mut self.texturas), Some(&self.vertices_textura), Some(&self.tex_map)) // NOTE: Por enquanto só
    }

    fn get_text(&mut self) -> (bool, Option<&mut Vec<Text<'a>>>) {
        (false, None)
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
                    /*
                    let state = PlayState::new("teste".to_string(), &mut *self.app) as Box<dyn State>;
                    */

                    let state = MapSelectorState::new(/*&mut *self.app*/) as Box<dyn for <'b> State<'b>>;
                    (*self.app).change_state(state);
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
