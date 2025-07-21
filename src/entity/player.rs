
pub struct Player {
    pub position: [f32; 3],
    pub mira: [f32; 3],
    fov: i32,
}

impl Player {
    pub fn new() -> Self {
        
        let position = [0.0, 5.0, 0.0];

        let mira = [5.0, 5.0, 0.0];

        let fov = 30;

        Self{position, mira, fov}
    }

    pub fn rotateViewX (&mut self, angle: f32) {

        let x = self.mira[0];
        let z = self.mira[2];

        //println!("x: {}, z: {}, i: {}, rgb:{},{},{}", x, z, index, self.vertex[index+3], self.vertex[index+4], self.vertex[index+5]);

        let catAdj = x + self.position[0] * -1.0;
        let catOp  = z + self.position[2] * -1.0;

        let raio = libm::sqrtf(libm::powf(catOp, 2.0) + libm::powf(catAdj, 2.0));

        let ang_rad = libm::atan2f(catOp, catAdj);

        let angFinal = (ang_rad * 57.2958 - angle) / 57.2958; // rad * 57.2958 + angulo (graus), dps
                                                              // divide para ir em radianos dnv         
        //println!("ang: {}, angf: {}", ang_rad, angFinal);
        let mut newX = 0.0;
        let mut newZ = 0.0;

        // println!("sen: {}, cos: {}, ang: {}", sen, cos, angFinal);
        
        newX = libm::cosf(angFinal) * raio;
        newZ = libm::sinf(angFinal) * raio;

        //if ((newZ * 1000000.0).round() == 0.0) {
        //    newZ *= -1.0;
        //}

        self.mira[0] = newX;
        self.mira[2] = newZ;

        println!("*mira* x: {}, y: {}, z: {}", self.mira[0], self.mira[1], self.mira[2]);
    }

   pub fn rotateViewY (&mut self, angle: f32) {

        let y = self.mira[1];
        let z = self.mira[2];

        //println!("y: {}, z: {}, i: {}, rgb:{},{},{}", y, z, index, self.vertex[index+3], self.vertex[index+4], self.vertex[index+5]);

        let catOp = y + self.position[1] * -1.0;
        let catAdj  = z + self.position[2] * -1.0;

        let raio = libm::sqrtf(libm::powf(catOp, 2.0) + libm::powf(catAdj, 2.0));

        let ang_rad = libm::atan2f(catOp, catAdj);

        let angFinal = (ang_rad * 57.2958 + angle) / 57.2958; // rad * 57.2958 + angulo (graus), dps
                                                              // divide para ir em radianos dnv         
        println!("ang: {}, angf: {}", ang_rad, angFinal);
        let mut newY = 0.0;
        let mut newZ = 0.0;

        // println!("sen: {}, cos: {}, ang: {}", sen, cos, angFinal);
        
        newY = libm::sinf(angFinal) * raio;
        newZ = libm::cosf(angFinal) * raio;

        //if ((newZ * 1000000.0).round() == 0.0) {
        //    newZ *= -1.0;
        //}

        self.mira[1] = newY;
        self.mira[2] = newZ;
    }


}
