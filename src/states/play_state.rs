use crate::logical::mapa::Mapa;
use crate::logical::entity::Player;
use crate::resources::file_manager;

use crate::App;

use winit::dpi::{PhysicalPosition, PhysicalSize};

use winit::event::WindowEvent;
use winit::window::Window;

struct PlayState<M: Mapa> {
    mapa: M,
    player: Player,
    vetores: Vec<f32>,
    app_state: &mut AppState,
    cursor_grab: CursorGrab
}

impl PlayState {
    pub fn new(mapa: String, app_state: &mut AppState ) -> Self {
        let mapa = file_manager::get_map(mapa);
        let player = Player::new(mapa.get_start_position()); // N sei se retornar os 2 valores                                                     
                                                             // assim dá certo, vamos ver
        
        let vetores = vec![0.0];                             // futuramente vai ter q adicionar a GUI
        
        let cursor_grab = CursorGrabber::new(app_state.window.inner_size());

        app_state.window.set_cursor_visible(false);

        Self{mapa, player, vetores, app_state, cursor_grab}
    }
}

impl State for PlayState {
    pub fn get_vertices(&self) -> Vec<f32> {
        self.vetores
    }

    pub fn update(&mut self) {
        let vetores = Vec::<f32>::new();

        for i in 0..self.objetos.len() {
             
            if self.objetos[i].verify_on_screen((*self.player).position, (*self.player).mira) {
                let grap_rep = self.objetos[i].visual.as_ref().unwrap();
                vetores.extend(grap_rep.vertex.iter().cloned());
            }
        }

        self.vetores = vetores;

    }

    pub fn manage_keyboard_input(&mut self, event: WindowEvent) {
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
                        self.appState.window.set_cursor_visible(true);
                    },

                    &_ => {
                    },
                }
            } 
        }
    }

    pub fn manage_mouse_input(&mut self, evento: WindowEvent) {
        if !self.cursor_grab.is_lock {
            self.cursor_grab.change_lock(true);
            self.appState.window.set_cursor_visible(false);
        }
    }

    pub fn manage_mouse_movement(&mut self, position: PhysicalPosition<f64>) {
        let state = self.state.as_mut().unwrap();

        if self.cursor_grab.is_lock {
            self.player.change_view(position, self.cursor_grab.position);
            app_state.window.set_cursor_position(self.cursor_grab.position);
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
