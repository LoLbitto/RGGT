use ::gl::types::*;
use libm::atan2f;

pub struct Visual {
    pub vertex: Vec<f32>,
    pub tipo:   GLenum,
    main_axis_x:  f32,
    main_axis_y:  f32,
    main_axis_z:  f32,
}

impl Visual {
    pub fn new (pontos: Vec<f32>, map: &Vec<f32>, tipo: GLenum) -> Self {
        
        let mut vertex = map.to_vec();

        for i in 0..vertex.len() / 7 {
            let index = i * 7;

            vertex[index] = pontos[vertex[index] as usize];
            vertex[index+1] = pontos[vertex[index+1] as usize];
            vertex[index+2] = pontos[vertex[index+2] as usize];
            vertex[index+3] = pontos[vertex[index+3] as usize];

            //println!("{}, {}, {}, {}", vertex[index+2] / vertex[index+3], vertex[index+4], vertex[index+5], vertex[index+6],);
        }

        let mut big_x = 0.0;
        let mut small_x = 0.0;

        let mut big_y = 0.0;
        let mut small_y = 0.0;

        let mut big_z = 0.0;
        let mut small_z = 0.0;

        for i in 0..vertex.len() / 7 {
            let index = i * 7;

            let x = vertex[index];
            let y = vertex[index + 1];
            let z = vertex[index + 2];

            if x > big_x {
                big_x = x;
            } else if x < small_x {
                small_x = x;
            }

            if y > big_y {
                big_y = y;
            } else if y < small_y {
                small_y = y;
            }

            if z > big_z {
                big_z = z;
            } else if z < small_z {
                small_z = z;
            }

        }

        let main_axis_x = (big_x + small_x) / 2.0;
        let main_axis_y = (big_y + small_y) / 2.0;
        let main_axis_z = (big_z + small_z) / 2.0;
 
        Self {vertex, tipo, main_axis_x, main_axis_y, main_axis_z}
    }

    pub fn rotate_x (&mut self, angle: f32) {
        for i in 0..(self.vertex.len() / 7) {
            let index = i * 7;

            let x = self.vertex[index];
            let z = self.vertex[index+2];

            let vec_x = x + self.main_axis_x * -1.0;
            let vec_z  = z + self.main_axis_z * -1.0;

            let new_coords = rotacionar_ponto(vec_x, vec_z, angle * 57.2958);

            self.vertex[index] = new_coords[0];
            self.vertex[index+2] = new_coords[1];
            
        }
    }

   pub fn rotate_y (&mut self, angle: f32) {
        for i in 0..(self.vertex.len() / 7) {
            let index = i * 7;

            let vec_y = self.vertex[index+1];
            let vec_z = self.vertex[index+2];

            let new_coords = rotacionar_ponto(vec_z, vec_y, angle * 57.2958);

            self.vertex[index+1] = new_coords[1];
            self.vertex[index+2] = new_coords[0];
            
        }
    }
}

pub fn rotacionar_ponto(cos: f32, sen: f32, rad: f32) -> [f32; 2] {

    let raio = libm::sqrtf(libm::powf(sen, 2.0) + libm::powf(cos, 2.0));

    let ang_rad = atan2f(sen, cos);

    let ang_final = ang_rad + rad;         
    
    let new_cos = libm::cosf(ang_final) * raio;
    let new_sen = libm::sinf(ang_final) * raio;   

    return [new_cos, new_sen]
}
