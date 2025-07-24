
pub struct Player {
    pub position: [f32; 3],
    pub mira: [f32; 3],
    fov: i32,
}

impl Player {
    pub fn new() -> Self {
        
        let position = [5.0, 5.0, 0.0];

        let mira = [position[0] + 5.0, position[1], position[2]];

        let fov = 30;

        Self{position, mira, fov}
    }

    pub fn rotate_view_x (&mut self, angle: f32) {

        let x = self.mira[0];
        let z = self.mira[2];
 
        let cat_adj = x - self.position[0];
        let cat_op  = z - self.position[2];

        let raio = libm::sqrtf(libm::powf(cat_op, 2.0) + libm::powf(cat_adj, 2.0));

        let ang_rad = libm::atan2f(cat_op, cat_adj);

        let ang_final = (ang_rad * 57.2958 - angle) / 57.2958; // rad * 57.2958 + angulo (graus), dps
                                                              // divide para ir em radianos dnv         
        let mut new_x = 0.0;
        let mut new_z = 0.0;
        
        new_x = libm::cosf(ang_final) * raio;
        new_z = libm::sinf(ang_final) * raio;

        self.mira[0] = new_x + self.position[0];
        self.mira[2] = new_z + self.position[2];

        println!("*mira* x: {}, y: {}, z: {}", self.mira[0], self.mira[1], self.mira[2]);
    }

   pub fn rotate_view_y (&mut self, angle: f32) {

        let y = self.mira[1];
        let z = self.mira[2];

        let cat_op  = y - self.position[1];
        let cat_adj = z - self.position[2];

        let raio = libm::sqrtf(libm::powf(cat_op, 2.0) + libm::powf(cat_adj, 2.0));

        let ang_rad = libm::atan2f(cat_op, cat_adj);

        let ang_final = (ang_rad * 57.2958 + angle) / 57.2958; // rad * 57.2958 + angulo (graus), dps
                                                              // divide para ir em radianos dnv         
        println!("ang: {}, angf: {}", ang_rad, ang_final);
        let mut new_y = 0.0;
        let mut new_z = 0.0;

        new_y = libm::sinf(ang_final) * raio;
        new_z = libm::cosf(ang_final) * raio;

        self.mira[1] = new_y;
        self.mira[2] = new_z;
    }

    pub fn move_relative_x(&mut self, speed: f32) {
        let mira_x = self.mira[0] - self.position[0];
        let mira_z = self.mira[2] - self.position[2];

        let hip = libm::sqrtf(libm::powf(mira_x, 2.0) + libm::powf(mira_z, 2.0));
    
        let sen = mira_z / hip;
        let cos = mira_x / hip;

        let x = cos * speed;
        let z = sen * speed;

        self.position[0] += x;
        self.position[2] += z;

        self.mira[0] += x;
        self.mira[2] += z;
    }
}
