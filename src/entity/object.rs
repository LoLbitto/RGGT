use crate::graphic::visualrep::Visual;

pub struct Object {
    points: Vec<f32>,
    visual: Visual,
}

impl Object {
    
    pub fn new(vec: Vec<f32>) -> Self {
        let visual = Visual::new(vec, ::gl::TRIANGLES);
        let points = vec![10.0, 10.0, 10.0,
                           5.0,  0.0,  5.0,
                          15.0,  0.0,  5.0,
                          15.0,  0.0, 15.0,
                           5.0,  0.0, 15.0,
                          ];

        Self{points, visual}
    }

    pub fn verifyOnScreen(& self, position: [f32; 3], mira: [f32; 3]) -> Vec<f32> {
        let x_factor = position[0] - mira[0];
        let z_factor = position[2] - mira[2]; 
    
        let mut points_shown = Vec::<f32>::new();

        for i in 0..self.points.len() {
            let index = i * 3;

            let x_ratio = position[0] - self.points[index];
            let z_ratio = position[0] - self.points[index+2];

            if (z_ratio / x_ratio < 1.0 && z_ratio / x_ratio > -1.0){

                if ((((x_ratio <= 0.0) && (x_factor <= 0.0)) || ((x_ratio >= 0.0) && (x_factor >= 0.0))) && (((z_ratio <= 0.0) && (z_factor <= 0.0)) || ((z_ratio >= 0.0) && (z_factor >= 0.0)))) {
                    for j in 0..2 {
                        points_shown.push(self.points[index+j]);
                    }
                }
            }
        }

        points_shown
    }
}
