use crate::ui::hitbox::Hitbox;
use winit::dpi::{PhysicalPosition, PhysicalSize};

struct Button {
    points: [f32; 8],
    hitbox: Hitbox,
    pub id: u32,
}

impl Button {
    pub fn new(pos: [f32; 2], width: f32, height: f32, id: u32) -> Self {
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

        let screen_size = PhysicalSize::<u32>::new(0, 0);

        let hitbox = Hitbox::new(points, screen_size);

        Self{points, hitbox, id}
    }

    pub fn update_screen_size(&mut self, screen_size: PhysicalSize<u32>) {
        let new_position = None;
        self.hitbox.update_position(Some(screen_size), new_position);
    }

    pub fn verify_inside(&self, position: PhysicalPosition<u32>) -> bool {
        self.hitbox.contains(position)
    }
}
