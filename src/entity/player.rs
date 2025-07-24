use winit::dpi::PhysicalPosition;
use crate::graphic::visualrep::rotacionar_ponto_x;

pub struct Player {
    pub position: [f32; 3],
    pub mira: [f32; 3],
    fov: i32,
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
}

impl Player {
    pub fn new() -> Self {
        
        let position = [5.0, 5.0, 0.0];

        let mira = [position[0] + 5.0, position[1], position[2]];

        let fov = 30;

        Self{position, mira, fov, w: false, a: false, s: false, d: false}
    }

    pub fn update(&mut self) {
        if self.w {
            self.move_relative_z(1.0);
        } else if self.s {
            self.move_relative_z(-1.0);
        } 

        if self.a {
            self.move_relative_x(1.0);
        } else if self.d {
            self.move_relative_x(-1.0);
        }
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

        let x = self.mira[0];
        let y = self.mira[1];
        let z = self.mira[2];

        let x_ratio = x - self.position[0];
        let y_ratio = y - self.position[1];
        let z_ratio = z - self.position[2];

        let hip = libm::sqrtf(libm::powf(x_ratio, 2.0) + libm::powf(z_ratio, 2.0));

        let sen = z_ratio / hip; // se tiver totalmente no Z, só levamos em consideração ele
        let cos = x_ratio / hip; // mesmo aqui
        
        let x_y = rotacionar_ponto_x(x_ratio, y_ratio, angle);
        let z_y = rotacionar_ponto_x(z_ratio, y_ratio, angle);

        let new_y = x_y[1] * cos + z_y[1] * sen;

        self.mira[1] = new_y;
    }

    pub fn move_relative_z(&mut self, speed: f32) {
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

    pub fn move_relative_x(&mut self, speed: f32) {
        let mira_x = self.mira[0] - self.position[0];
        let mira_z = self.mira[2] - self.position[2];

        let hip = libm::sqrtf(libm::powf(mira_x, 2.0) + libm::powf(mira_z, 2.0));
    
        let sen = mira_z / hip;
        let cos = mira_x / hip;

        let z = cos * speed;
        let x = sen * speed;

        self.position[0] += x;
        self.position[2] += z;

        self.mira[0] += x;
        self.mira[2] += z;
    }

    pub fn change_view (&mut self, position: PhysicalPosition<f64>, meio: PhysicalPosition<f64>) {
        let width_ratio = meio.x - position.x;
        let height_ratio = meio.y - position.y;

        let mut x_inc = width_ratio / width_ratio;
        let mut y_inc = height_ratio / height_ratio; // Parece mais ez definir como 1, mas e se for 0,
                                                 // daí é mais um if, mô empenho slk. Assim é mais
                                                 // prático

        if width_ratio < 1.0 {
            x_inc *= -1.0;
        }

        if height_ratio < 1.0 {
            y_inc *= -1.0;
        }
        
        self.rotate_view_x(-width_ratio as f32);
        //self.rotate_view_y(-height_ratio as f32);

    }
}
