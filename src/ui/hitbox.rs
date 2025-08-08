use winit::dpi::{PhysicalPosition, PhysicalSize};

pub struct Hitbox {
    points_position: [f32; 8], // 4 Pontos com X e Y
    screen_size: PhysicalSize<u32>,
    real_position: [f32; 8],   // Posição real com relação à tela
}

impl Hitbox {

    pub fn new(points_position: [f32; 8], screen_size: PhysicalSize<u32>) -> Self {
        let real_position = [0.0; 8];

        Self{points_position, screen_size, real_position}
    }

    pub fn update_position(&mut self, screen_size: Option<PhysicalSize<u32>>, new_position: Option<[f32; 8]>) {
        match screen_size {
            Some(p) => {
                println!("Entrou");
                self.screen_size = screen_size.unwrap();
            },

            None => {}
        }

        if new_position != None {
            self.points_position = new_position.unwrap();
        }

        self.real_position = Self::calculate_real_position(self.points_position, self.screen_size);
    }

    pub fn contains(&self, position: PhysicalPosition<f64>) -> bool {
        
        let (pos_x, pos_y) = (position.x, position.y);

        let (mut big_x, mut small_x, mut big_y, mut small_y) = (0.0, 0.0, 0.0, 0.0);

        for i in 0..self.real_position.len()/2 {
            let index = i * 2;

            let point_x = self.real_position[index] as f64;
            let point_y = self.real_position[index+1] as f64;

            if point_x > big_x {
                big_x = point_x;
            } 

            if point_x < small_x || small_x == 0.0{
                small_x = point_x;
            }

            if point_y > big_y {
                big_y = point_y;
            }
            
            if point_y < small_y || small_y == 0.0{
                small_y = point_y;
            }
        }
        
        let (mut is_inside_x, mut is_inside_y) = (false, false);

        if pos_x <= big_x && pos_x >= small_x {
            is_inside_x = true;
        } 

        if pos_y <= big_y && pos_y >= small_y {
            is_inside_y = true;
        }

        if is_inside_x && is_inside_y {
            true
        } else {
            false
        }
    }

    fn calculate_real_position(pontos: [f32; 8], screen_size: PhysicalSize<u32>) -> [f32; 8] {
        let mut pontos_final = pontos;
        let width = (screen_size.width / 2) as f32;
        let height = (screen_size.height / 2) as f32;
    
        for i in 0..pontos.len()/2 {
            let index = i*2;

            pontos_final[index] = pontos[index] * width + width; // Ponto vai
                                                                 // de
                                                                 // -1 (0)
                                                                 // a 0 (metade) 
                                                                 // a 1 (final)
            pontos_final[index+1] = -pontos[index+1] * height + height; 
        }

        pontos_final
    }
}
