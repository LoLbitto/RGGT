use ::gl::types::*;
use libm::atan2f;

pub struct Visual {
    pub vertex: Vec<f32>,
    pub tipo:   GLenum,
    mainAxisX:  f32,
    mainAxisY:  f32,
    mainAxisZ:  f32,
}

// impl Copy for Visual { }

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

        //println!("mainAxisX: {}, mainAxisY: {}, mainAxisZ: {}", mainAxisX, mainAxisY, mainAxisZ);

        Self {vertex, tipo, mainAxisX, mainAxisY, mainAxisZ}
    }

    pub fn rotateX (&mut self, angle: f32) {
        for i in 0..(self.vertex.len() / 6) {
            let index = i * 6;

            let x = self.vertex[index];
            let z = self.vertex[index+2];

            let vec_x = x + self.mainAxisX * -1.0;
            let vec_z  = z + self.mainAxisZ * -1.0;

            let newCoords = rotacionarPontoX(vec_x, vec_z, angle * 57.2958);

            self.vertex[index] = newCoords[0];
            self.vertex[index+2] = newCoords[1];
            
        }
    }

   pub fn rotateY (&mut self, angle: f32) {
        for i in 0..(self.vertex.len() / 6) {
            let index = i * 6;

            let vecY = self.vertex[index+1];
            let vecZ = self.vertex[index+2];

            let newCoords = rotacionarPontoY(vecY, vecZ, angle * 57.2958);

            self.vertex[index+1] = newCoords[0];
            self.vertex[index+2] = newCoords[1];
            
        }
    }
}

pub fn rotacionarPontoX(vecX: f32, vecZ: f32, rad: f32) -> [f32; 2] {

    let raio = libm::sqrtf(libm::powf(vecZ, 2.0) + libm::powf(vecX, 2.0));

    let ang_rad = atan2f(vecZ, vecX);

    let angFinal = ang_rad + rad;         
    
    let newX = libm::cosf(angFinal) * raio;
    let newZ = libm::sinf(angFinal) * raio;   

    return [newX, newZ]
}

pub fn rotacionarPontoY(vecY: f32, vecZ: f32, rad: f32) -> [f32; 2] {

    let raio = libm::sqrtf(libm::powf(vecY, 2.0) + libm::powf(vecZ, 2.0));

    let ang_rad = atan2f(vecY, vecZ);

    let angFinal = ang_rad + rad;         
    
    let newY = libm::sinf(angFinal) * raio;
    let newZ = libm::cosf(angFinal) * raio;   

    return [newY, newZ]
}
