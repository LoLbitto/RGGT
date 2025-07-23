use crate::graphic::visualrep::Visual;
use crate::graphic::visualrep::rotacionarPontoX;
use crate::graphic::visualrep::rotacionarPontoY;

use ::gl::types::*;

pub struct Object {
    points: Vec<f32>,
    map: Vec<f32>,
    pub is_viewed: bool,
    pub visual: Option<Visual>,
}

// impl Copy for Object { }

impl Object {
    
    pub fn new() -> Self {
        let points = vec![
                          16.0,  5.0, 0.0,
                          13.0,  0.0,-3.0,
                          13.0,  0.0, 3.0,
                          19.0,  0.0,-3.0,
                          19.0,  0.0, 3.0,
                         ];
        let map = vec![
                        0.0, 1.0, 2.0, 3.0, 0.0, 0.0, 1.0,
                        4.0, 5.0, 6.0, 7.0, 0.0, 0.0, 1.0,
                        8.0, 9.0, 10.0, 11.0, 0.0, 0.0, 1.0,

                        0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 0.0,
                        8.0, 9.0, 10.0, 11.0, 0.0, 1.0, 0.0,
                        16.0, 17.0, 18.0, 19.0, 0.0, 1.0, 0.0,

                        0.0, 1.0, 2.0, 3.0, 1.0, 0.0, 0.0,
                        12.0, 13.0, 14.0, 15.0, 1.0, 0.0, 0.0,
                        16.0, 17.0, 18.0, 19.0, 1.0, 0.0, 0.0,

                        0.0, 1.0, 2.0, 3.0, 1.0, 1.0, 1.0,
                        4.0, 5.0, 6.0, 7.0, 1.0, 1.0, 1.0,
                        12.0, 13.0, 14.0, 15.0, 1.0, 1.0, 1.0,

                        4.0, 5.0, 6.0, 7.0, 1.0, 0.0, 1.0,
                        8.0, 9.0, 10.0, 11.0, 1.0, 0.0, 1.0,
                        12.0, 13.0, 14.0, 15.0, 1.0, 0.0, 1.0,

                        16.0, 17.0, 18.0, 19.0, 1.0, 0.0, 1.0,
                        8.0, 9.0, 10.0, 11.0, 1.0, 0.0, 1.0,
                        12.0, 13.0, 14.0, 15.0, 1.0, 0.0, 1.0,
                      ];

        Self{points, map, is_viewed: false, visual: None}
    }

    pub fn verifyOnScreen(&mut self, position: [f32; 3], mira: [f32; 3]) -> bool {
        let x_factor = mira[0] - position[0];
        let y_factor = mira[1] - position[1];
        let z_factor = mira[2] - position[2]; 

        let mut is_on_screen = false;

        for i in 0..self.points.len() / 3 {
            let index = i * 3;

            let x_ratio = self.points[index] - position[0];
            let y_ratio = self.points[index+1] - position[1];
            let z_ratio = self.points[index+2] - position[2];

            if (z_ratio / x_ratio <= 1.0 && z_ratio / x_ratio >= -1.0) && ((y_ratio / z_ratio <= 1.0 && y_ratio / z_ratio >= -1.0) || (y_ratio / x_ratio <= 1.0 && y_ratio / x_ratio >= -1.0)) {

                if (((x_ratio <= 0.0) && (x_factor <= 0.0)) || ((x_ratio >= 0.0) && (x_factor >= 0.0))) && (((z_ratio <= 0.0) && (z_factor <= 0.0)) || ((z_ratio >= 0.0) && (z_factor >= 0.0))) {
                    is_on_screen = true;
                    println!("sim");
                    break;
                } else {
                    println!("não");
                }
            } else {
                println!("xr: {}, yr: {}, zr: {}", x_ratio, y_ratio, z_ratio);
                println!("xf: {}, yf: {}, zf: {}", x_factor, y_factor, z_factor);
            }
        }
        
        if is_on_screen {
            self.is_viewed = true;

            let mut posicao_relativa = Vec::<f32>::new();

            for i in 0..self.points.len() / 3 {
                
                let index = i * 3;

                let x_ratio = self.points[index] - position[0];
                let y_ratio = self.points[index+1] - position[1];
                let z_ratio = self.points[index+2] - position[2];

                let new_x_z = rotacionarPontoX(x_ratio, z_ratio, libm::atan2f(mira[0], mira[2]));
                let new_y = rotacionarPontoY(y_ratio, z_ratio, libm::atan2f(mira[2], mira[1]))[0];

                let hipotenusa = libm::sqrtf(libm::powf(new_x_z[0], 2.0) + libm::powf(new_x_z[1], 2.0));

                let mut visual_w = hipotenusa / 10.0;

                let mut mod_vis_w = visual_w;

                if (mod_vis_w < 0.0) {
                    mod_vis_w *= -1.0;
                }

                let mut visual_x = new_x_z[0] / 10.0 * mira[0] / libm::sqrtf(libm::powf(mira[0], 2.0) + libm::powf(mira[2], 2.0)) + new_x_z[0] / libm::sqrtf(libm::powf(new_x_z[0], 2.0) + libm::powf(new_x_z[1], 2.0)); //10 Gu = 1 no opengl
                let mut visual_y = new_y * mod_vis_w / 10.0;
                let mut visual_z = new_x_z[1] * mod_vis_w / 10.0;

                if (visual_z > 1.0) {
                    let diference = visual_z - 1.0;
                    visual_w += visual_z - diference;
                    visual_z = diference;
                    println!("Entrou no 1.0");
                } else if (visual_z < -1.0) {
                    let diference = visual_z + 1.0;
                    visual_w += visual_z - diference;
                    visual_z = diference;
                    println!("Entrou no -1.0");
                }

                visual_z *= libm::sinf(libm::atan2f(mira[0], mira[2]));
                //visual_x /= libm::cosf(libm::atan2f(mira[2], mira[0]));
                visual_y /= visual_w;

                posicao_relativa.push(visual_x);
                posicao_relativa.push(visual_y);
                posicao_relativa.push(visual_z);
                posicao_relativa.push(visual_w);

                println!("x: {}, y: {}, z: {}, w: {}", visual_x, visual_y, visual_z, visual_w);
            }

            self.visual = Some(Visual::new(posicao_relativa, &self.map, gl::TRIANGLES));

            return true
        } else {
            self.is_viewed = false;
            return false
        }
    }
}
