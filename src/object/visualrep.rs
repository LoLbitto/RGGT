use ::gl::types::*;
use libm::atan2f;

pub struct Visual {
    pub vertex: Vec<f32>,
    pub tipo:   GLenum,
    mainAxisX:  f32,
    mainAxisY:  f32,
    mainAxisZ:  f32,
}

impl Visual {
    pub fn new (vertex: Vec<f32>, tipo: GLenum) -> Self {
        
        let mut bigX = 0.0;
        let mut smallX = 0.0;

        let mut bigY = 0.0;
        let mut smallY = 0.0;

        let mut bigZ = 0.0;
        let mut smallZ = 0.0;

        for i in 0..vertex.len() / 6 {
            let index = i * 6;

            let x = vertex[index];
            let y = vertex[index + 1];
            let z = vertex[index + 2];

            if x > bigX {
                bigX = x;
            } else if x < smallX {
                smallX = x;
            }

            if y > bigY {
                bigY = y;
            } else if y < smallY {
                smallY = y;
            }

            if z > bigZ {
                bigZ = z;
            } else if z < smallZ {
                smallZ = z;
            }

        }

        let mainAxisX = (bigX + smallX) / 2.0;
        let mainAxisY = (bigY + smallY) / 2.0;
        let mainAxisZ = (bigZ + smallZ) / 2.0;

        println!("mainAxisX: {}, mainAxisY: {}, mainAxisZ: {}", mainAxisX, mainAxisY, mainAxisZ);

        Self {vertex, tipo, mainAxisX, mainAxisY, mainAxisZ}
    }

    pub fn rotateX (&mut self, angle: f32) {
        for i in 0..(self.vertex.len() / 6) {
            let index = i * 6;

            let x = self.vertex[index];
            let z = self.vertex[index+2];

            println!("x: {}, z: {}, i: {}", x, z, index);

            let catAdj = x + self.mainAxisX * -1.0;
            let catOp  = z + self.mainAxisZ * -1.0;

            let raio = libm::sqrtf(libm::powf(catOp, 2.0) + libm::powf(catAdj, 2.0));

            let angRad = atan2f(catOp,catAdj);
            let angFinal = (angRad * 57.2958 + angle) / 57.2958; // rad * 57.2958 + angulo (graus), dps
                                                             // divide para ir em radianos dnv
            let newX = libm::cosf(angFinal) * raio;
            let newZ = libm::sinf(angFinal) * raio;

            self.vertex[index] = newX;
            self.vertex[index+2] = newZ;
            
        }
    }
}
