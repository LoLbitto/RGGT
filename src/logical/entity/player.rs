use winit::dpi::PhysicalPosition;
use crate::graphic::visualrep::rotacionar_ponto;

pub struct Player {
    pub position: [f32; 3],
    pub mira: [f32; 3],
    fov: i32,
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
}

unsafe impl Send for Player {}

impl Player {
    pub fn new(position: [f32; 3], mira: [f32;3]) -> Self {
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
 
        let x_ratio = x - self.position[0];
        let z_ratio  = z - self.position[2];

        let new_x_z = rotacionar_ponto(x_ratio, z_ratio, angle);

        self.mira[0] = new_x_z[0] + self.position[0];
        self.mira[2] = new_x_z[1] + self.position[2];

        //println!("*mira* x: {}, y: {}, z: {}", self.mira[0], self.mira[1], self.mira[2]);
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
        
        let x_y = rotacionar_ponto(x_ratio, y_ratio, angle * cos);
        let z_y = rotacionar_ponto(z_ratio, y_ratio, angle * sen);    
        
        let new_y = x_y[1] + z_y[1] - y_ratio + self.position[1];

        self.mira[0] = x_y[0] + self.position[0];
        self.mira[1] = new_y;
        self.mira[2] = z_y[0] + self.position[2];

        //println!("*mira* x: {}, y: {}, z: {}", self.mira[0], self.mira[1], self.mira[2]);
    }

    pub fn move_relative_z(&mut self, speed: f32) {
        let mira_ratio_x = self.mira[0] - self.position[0];
        let mira_ratio_z = self.mira[2] - self.position[2];

        let hip = libm::sqrtf(libm::powf(mira_ratio_x, 2.0) + libm::powf(mira_ratio_z, 2.0));
    
        let sen = mira_ratio_z / hip;
        let cos = mira_ratio_x / hip;

        let x = cos * speed;
        let z = sen * speed;

        self.position[0] += x;
        self.position[2] += z;

        self.mira[0] += x;
        self.mira[2] += z;
    
        println!("x: {}, y: {}, z: {}", self.position[0], self.position[1], self.position[2]);
    }

    pub fn move_relative_x(&mut self, speed: f32) {
        let mira_ratio_x = self.mira[0] - self.position[0];
        let mira_ratio_z = self.mira[2] - self.position[2];

        let new_mira_x_z = rotacionar_ponto(mira_ratio_x, mira_ratio_z, libm::asinf(1.0));

        let hip = libm::sqrtf(libm::powf(new_mira_x_z[0], 2.0) + libm::powf(new_mira_x_z[1], 2.0));
    
        let sen = new_mira_x_z[1] / hip;
        let cos = new_mira_x_z[0] / hip;

        let x = cos * speed;
        let z = sen * speed;

        self.position[0] += x;
        self.position[2] += z;

        self.mira[0] += x;
        self.mira[2] += z;

        println!("x: {}, y: {}, z: {}", self.position[0], self.position[1], self.position[2]);
    }

    pub fn change_view (&mut self, position: PhysicalPosition<f64>, meio: PhysicalPosition<f64>) {
        let width_ratio = meio.x - position.x;
        let height_ratio = meio.y - position.y;

        self.rotate_view_x(width_ratio as f32 / 300.0 );
        self.rotate_view_y(height_ratio as f32 / 300.0 );

    }
}
