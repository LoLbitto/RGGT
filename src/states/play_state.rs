use crate::logical::mapa::Mapa;
use crate::logical::entity::player::Player;
use crate::resources::file_manager;
use crate::app::AppState;
use crate::states::State;

use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::KeyEvent;
use winit::event::WindowEvent;
use winit::window::Window;
use winit::event::ElementState;

pub struct PlayState {
    mapa: Box<dyn Mapa>,
    player: Player,
    vetores: Vec<f32>,
    app_state: *const AppState,
    cursor_grab: CursorGrabber
}

impl PlayState {
    pub fn new(data: String, app_state: &mut AppState ) -> Box<Self> {
        let mapa = Box::new(file_manager::get_map(data));
        let (player_pos, mira) = mapa.get_start_position();
        let player = Player::new(*player_pos, *mira);                 
        let vetores = vec![0.0];                             // futuramente vai ter q adicionar a GUI
        
        let cursor_grab = CursorGrabber::new(app_state.window.inner_size());

        app_state.window.set_cursor_visible(false);

        Box::new(Self{mapa, player, vetores, app_state, cursor_grab})
    }

}

impl State for PlayState {

    fn get_vertices(&self) -> &Vec<f32> {
        &self.vetores
    }

    fn update(&mut self) {
        let mut vetores = Vec::<f32>::new();

        for i in 0..self.mapa.get_objects().len() {
             
            if self.mapa.get_objects()[i].verify_on_screen(self.player.position, self.player.mira) {
                let grap_rep = self.mapa.get_objects()[i].visual.as_ref().unwrap();
                vetores.extend(grap_rep.vertex.iter().cloned());
            }
        }

        self.vetores = vetores;
        self.player.update();

    }

    fn manage_keyboard_input(&mut self, event: KeyEvent) {
        if !event.repeat {
            let char = event.logical_key.to_text();

            if char != None {
                match char.unwrap() {
                    "w" => {
                        if event.state == ElementState::Pressed {
                            self.player.w = true;
                        } else if event.state == ElementState::Released {
                            self.player.w = false;
                        }
                    },

                    "a" => {
                        if event.state == ElementState::Pressed {
                            self.player.a = true;
                        } else if event.state == ElementState::Released {
                            self.player.a = false;
                        }
                    },
                    
                    "s" => {
                        if event.state == ElementState::Pressed {
                            self.player.s = true;
                        } else if event.state == ElementState::Released {
                            self.player.s = false;
                        }
                    },

                    "d" => {
                        if event.state == ElementState::Pressed {
                            self.player.d = true;
                        } else if event.state == ElementState::Released {
                            self.player.d = false;
                        }
                    },
                    
                    "\x1b" => {
                        self.cursor_grab.change_lock(false);
                        unsafe {
                            (*self.app_state).window.set_cursor_visible(true);
                        }
                    },

                    &_ => {
                    },
                }
            } 
        }
    }

    fn manage_mouse_input(&mut self, evento: WindowEvent) {
        if !self.cursor_grab.is_lock {
            self.cursor_grab.change_lock(true);
            unsafe {
                (*self.app_state).window.set_cursor_visible(false);
            }
        }
    }

    fn manage_mouse_movement(&mut self, position: PhysicalPosition<f64>) {

        if self.cursor_grab.is_lock {
            self.player.change_view(position, self.cursor_grab.position);
            unsafe {
                (*self.app_state).window.set_cursor_position(self.cursor_grab.position);
            }
        }
    }

    fn manage_window_resize(&mut self, size: PhysicalSize<u32>) {
        self.cursor_grab.window_resized(size);
    }
}

struct CursorGrabber {
    window_size: [f64; 2],
    pub position: PhysicalPosition<f64>,
    pub is_lock: bool
}

impl CursorGrabber {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        let window_size = [size.width as f64, size.height as f64];
        let position = Self::calc_position(size);
        let is_lock = true;

        Self {window_size, position, is_lock}
    }

    pub fn calc_position(size: PhysicalSize<u32>) -> PhysicalPosition<f64>{
        let mid_width = size.width as f64 / 2.0;
        let mid_height = size.height as f64 / 2.0;

        PhysicalPosition::<f64>::new(mid_width, mid_height)
    }

    pub fn change_lock(&mut self, lock: bool) {
        self.is_lock = lock;
    }

    pub fn window_resized(&mut self, size: PhysicalSize<u32>) {
        self.position = Self::calc_position(size);
    }
}
