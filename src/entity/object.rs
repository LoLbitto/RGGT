use crate::graphic::visualrep::Visual;

pub struct Object {
    points: Vec<f32>,
    pub is_viewed: bool,
    pub visual: Option<Visual>,
}

// impl Copy for Object { }

impl Object {
    
    pub fn new() -> Self {
        let points = vec![10.0, 5.0, 10.0,
                           1.0,  5.0,  0.0,
                           0.0,  5.0,  1.0,
                           0.0,  5.0, 1.0,
                           5.0,  5.0, 15.0,
                          ];

        Self{points, is_viewed: false, visual: None}
    }

    pub fn verifyOnScreen(&mut self, position: [f32; 3], mira: [f32; 3]) -> bool {
        let x_factor = mira[0] - position[0];
        let y_factor = mira[1] - position[1];
        let z_factor = mira[2] - position[2]; 

        let mut is_on_screen = false;

        for i in 0..self.points.len() / 3 {
            let index = i;

            let x_ratio = self.points[index] - position[0];
            let y_ratio = self.points[index+1] - position[1];
            let z_ratio = self.points[index+2] - position[2];

            if ((z_ratio / x_ratio <= 1.0 && z_ratio / x_ratio >= -1.0) && (y_ratio / z_ratio <= 1.0 && y_ratio / z_ratio >= -1.0)){

                if ((((x_ratio <= 0.0) && (x_factor <= 0.0)) || ((x_ratio >= 0.0) && (x_factor >= 0.0))) && (((z_ratio <= 0.0) && (z_factor <= 0.0)) || ((z_ratio >= 0.0) && (z_factor >= 0.0)))) {
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
        
        if (is_on_screen) {
            self.is_viewed = true;

            let mut posicao_relativa = Vec::<f32>::new();

            for i in 0..self.points.len() / 3 {
                let index = i;

                let x_ratio = position[0] - self.points[index];
                let y_ratio = position[1] - self.points[index+1];
                let z_ratio = position[2] - self.points[index+2];
                
                let angulo_x = libm::atan2f(z_ratio, x_ratio);
                let angulo_y = libm::atan2f(y_ratio, z_ratio);

                let z = libm::sinf(angulo_x);
                let x = libm::cosf(angulo_x);
                let y = libm::sinf(angulo_y);

                println!("x: {}, y: {}, z: {}", x, y, z);
            }

            return true
        } else {
            self.is_viewed = false;
            return false
        }
    }
}
