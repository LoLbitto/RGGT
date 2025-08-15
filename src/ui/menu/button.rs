use crate::ui::hitbox::Hitbox;
use crate::graphic::texture::Texture;

use winit::dpi::{PhysicalPosition, PhysicalSize};

const DEFAULT_Z: f32 = 0.5;
const DEFAULT_W: f32 = 1.0;

pub struct Button {
    pub points: [f32; 8],
    hitbox: Hitbox,
    texture: Option<Texture>,
    vertices: Vec<f32>,
    pub has_texture: bool,
    pub id: u32,
}

impl Button {
    pub fn new(pos: [f32; 2], width: f32, height: f32, texture_name: &str , id: u32) -> Self {
        let mut points = [0.0; 8];

        let x = pos[0];
        let y = pos[1]; // Declarar para maior legibilidade futura

        for i in 0..4 {
            let index = i * 2;
            match i {
                0 => {
                    points[index] = x;
                    points[index+1] = y;
                }, 

                1 => {
                    points[index] = x;
                    points[index+1] = y + height;
                },

                2 => {
                    points[index] = x + width;
                    points[index+1] = y;
                },

                3 => {
                    points[index] = x + width;
                    points[index+1] = y + height;
                },

                _ => {}
            }
        }

        let mut has_texture = false;
        let mut texture = None;

        if texture_name != "" {
            has_texture = true;
            texture = Some(Texture::new(texture_name));
        }

        let vertices = Self::calculate_vertices(points, has_texture);

        let screen_size = PhysicalSize::<u32>::new(0, 0);

        let hitbox = Hitbox::new(points, screen_size);

        Self{points, hitbox, texture, vertices, has_texture, id}
    }

    pub fn update_screen_size(&mut self, screen_size: PhysicalSize<u32>) {
        let new_position = None;
        self.hitbox.update_position(Some(screen_size), new_position);
    }

    pub fn verify_inside(&self, position: PhysicalPosition<f64>) -> bool {
        self.hitbox.contains(position)
    }

    pub fn get_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }

    pub fn get_texture(&mut self) -> &mut Texture {
        self.texture.as_mut().unwrap()
    }

    fn calculate_vertices(points: [f32; 8], has_texture: bool) -> Vec<f32> {
        let mut vertices = Vec::<f32>::new();
        
        let (red, green, blue) = (1.0, 1.0, 1.0);

        for i in 0..2 {
            for j in i..points.len() / 2 - 1{
                let index = j * 2;
                let (x, y) = (points[index], points[index+1]);

                vertices.push(x);
                vertices.push(y);
                vertices.push(DEFAULT_Z);
                vertices.push(DEFAULT_W);
                vertices.push(red);
                vertices.push(green);
                vertices.push(blue);

                if has_texture {
                    match j {
                        0 => {
                            vertices.push(0.0);
                            vertices.push(0.0);
                        },

                        1 => {
                            vertices.push(0.0);
                            vertices.push(1.0);
                        },

                        2 => {
                            vertices.push(1.0);
                            vertices.push(0.0);
                        },

                        3 => {
                            vertices.push(1.0);
                            vertices.push(1.0);
                        },

                        _ => {}
                    }
                }
            }
        }

        vertices
    }
}
