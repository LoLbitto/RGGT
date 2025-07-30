use crate::graphic::visualrep::Visual;
use crate::graphic::visualrep::rotacionar_ponto_x;
use crate::graphic::visualrep::rotacionar_ponto_y;
use crate::resources::file_manager;

pub struct Object {
    points: Vec<f32>,
    map: Vec<f32>,
    pub is_viewed: bool,
    pub visual: Option<Visual>,
}

impl Object {
    
    pub fn new(model: String, position: Vec<f32>, scale: f32) -> Self {
        let (mut points, mut map) = file_manager::get_object(model);

        for j in 0..points.len()/3 {
            let i = j * 3;

            points[i] *= scale;
            points[i+1] *= scale;
            points[i+2] *= scale;


            points[i] += position[0];
            points[i+1] += position[1];
            points[i+2] += position[2];
        
            println!("x: {}, y: {}, z: {}", points[i], points[i+1], points[i+2])
        } 

        println!("Pontos:");

        for ponto in &points {
            println!("{}", ponto);
        }

        println!("Map:");

        for ponto in &map {
            println!("{}", ponto);
        }

        Self{points, map, is_viewed: false, visual: None}
    }

    pub fn verify_on_screen(&mut self, position: [f32; 3], mira: [f32; 3]) -> bool {
        let x_factor = mira[0] - position[0];
        let y_factor = mira[1] - position[1];
        let z_factor = mira[2] - position[2];

        let hip_v = libm::sqrtf(libm::powf(x_factor, 2.0) + libm::powf(z_factor, 2.0));

        let mut is_on_screen = false;

        for i in 0..self.points.len() / 3 {
            let index = i * 3;

            let x_ratio = self.points[index] - position[0];
            let y_ratio = self.points[index+1] - position[1];
            let z_ratio = self.points[index+2] - position[2];

            let new_x_z = rotacionar_ponto_x(x_ratio, z_ratio, libm::atan2f(x_factor, z_factor));

            let hip = libm::sqrtf(libm::powf(new_x_z[0], 2.0) + libm::powf(new_x_z[1], 2.0));

            //println!("{}, {}", new_x_z[0], new_x_z[1]);

            if (new_x_z[1] / new_x_z[0] >= 1.0 || new_x_z[1] / new_x_z[0] <= -1.0) && new_x_z[1] / hip > 0.0 {
                is_on_screen = true; 
                break;           
            } 
        }
        
        if is_on_screen {
            self.is_viewed = true;

            let mut posicao_relativa = Vec::<f32>::new();

            for i in 0..self.points.len() / 3 {
                
                let index = i * 3;

                let x_ratio = self.points[index]   - position[0];
                let y_ratio = self.points[index+1] - position[1];
                let z_ratio = self.points[index+2] - position[2];

                let new_x_z = rotacionar_ponto_x(x_ratio, z_ratio, libm::atan2f(x_factor, z_factor));

                let z_factor_rot = rotacionar_ponto_x(x_factor, z_factor, libm::atan2f(x_factor, z_factor))[1];
                let hip_y = libm::sqrtf(libm::powf(y_factor, 2.0) + libm::powf(z_factor_rot, 2.0));

                let new_y_z = rotacionar_ponto_y(y_ratio, new_x_z[1], libm::asinf(y_factor / hip_y));

                let hipotenusa = libm::sqrtf(libm::powf(x_ratio, 2.0) + libm::powf(z_ratio, 2.0) + libm::powf(y_ratio, 2.0));

                let visual_w = hipotenusa;

                let visual_x = new_x_z[0] * 2.0;
                let visual_y = new_y_z[0] * 2.0;
                let visual_z = hipotenusa - 0.5;

                //println!("{}", visual_z);

                posicao_relativa.push(visual_x);
                posicao_relativa.push(visual_y);
                posicao_relativa.push(visual_z);
                posicao_relativa.push(visual_w);

            }

            self.visual = Some(Visual::new(posicao_relativa, &self.map, gl::TRIANGLES));

            return true
        } else {
            self.is_viewed = false;
            return false
        }
    }
}
