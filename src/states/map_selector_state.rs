use crate::states::State;
use crate::resources::file_manager::listing;
use crate::graphic::texture::Texture;

use crate::ui::text::Text;
use crate::ui::text::TextFabric;

use winit::event::KeyEvent;
use winit::keyboard::{PhysicalKey, KeyCode};
use winit::event::ElementState;
use winit::event::MouseButton;
use winit::dpi::{PhysicalPosition, PhysicalSize};

struct MapSelectorState<'a> {
    maps: Vec<String>,
    maps_text: Vec<Text<'a>>,
    text_fabric: TextFabric,
    selector: u32,
    key_manager: KeyManager,
    vertices: Vec<f32>
}

struct KeyManager {
    pub up: bool,
    pub down: bool
}

impl MapSelectorState<'_> {
    pub fn new() -> Self {
        let maps = listing::get_resource_list(listing::Resource::Map);
        let selector = 0;
        let key_manager = KeyManager {up: false, down: false};
        let vertices = vec![0.0];

        let mut text_fabric = TextFabric::new("MxPlus_IBM_MDA".to_owned());

        let mut maps_text = Vec::<Text>::new();

        Self{maps, maps_text, text_fabric, selector, key_manager, vertices}
    }
}

impl State for MapSelectorState<'_> {
    
    fn get_vertices (&self) -> &Vec<f32> {
        &self.vertices
    }

    fn get_textures(&mut self) -> (bool, Option<&mut Vec<*mut Texture>>, Option<& Vec<f32>>, Option<&Vec<u32>>) {
        (false, None, None, None) // NOTE: Por enquanto só
    }

    fn update(&mut self) {
        let down = self.key_manager.down;
        let up = self.key_manager.up ;
        match true {
            up => {
                if self.selector == self.maps.len() as u32 - 1 {
                    self.selector = 0;
                } else {
                    self.selector += 1;
                }
            },

            down => {
                if self.selector == 0 {
                    self.selector = self.maps.len() as u32 - 1;
                } else {
                    self.selector -= 1;
                }
            }
        }
    }

    fn manage_keyboard_input(&mut self, event: KeyEvent) {
        if !event.repeat {
            let tecla = event.physical_key;

            match tecla {
                PhysicalKey::Code(code) => {
                    match code {
                        KeyCode::ArrowUp => {
                            if event.state == ElementState::Pressed {
                                self.key_manager.up = true;
                            } else {
                                self.key_manager.up = false;
                            }
                        }

                        KeyCode::ArrowDown => {
                            if event.state == ElementState::Pressed {
                                self.key_manager.down = true;
                            } else {
                                self.key_manager.down = false;
                            }
                        },

                        _ => {}
                    }
                },

                _ => {}
            }
        }
    }

    fn manage_mouse_input(&mut self, button: MouseButton) {
    
    }

    fn manage_mouse_movement(&mut self, position: PhysicalPosition<f64>) {

    }
    
    fn manage_window_resize(&mut self, size: PhysicalSize<u32>) {

    }

}
