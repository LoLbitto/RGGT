use winit::dpi::{PhysicalPosition, PhysicalSize};

pub struct Hitbox {
    points_position: [f32; 8], // 4 Pontos com X e Y
    screen_size: PhysicalSize<u32>,
    real_position: [f32; 8],   // Posição real com relação à tela
}

impl Hitbox {

    pub fn new(points_position: [f32; 8], screen_size: PhysicalSize<u32>) -> Self {
        let real_position = Self::calculate_real_position(points_position, screen_size);

        Self{points_position, screen_size, real_position}
    }

    pub fn change_position(&mut self, screen_size: Option<PhysicalSize<u32>>, new_position: Option<[f32; 8]>) {
        if screen_size != None {
            self.screen_size = screen_size.unwrap();
        }

        if new_position != None {
            self.points_position = new_position.unwrap();
        }

        self.real_position = Self::calculate_real_position(self.points_position, self.screen_size);
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
            pontos_final[index+1] = pontos[index+1] * height + height; 
        }

        pontos_final
    }
}
