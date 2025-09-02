pub struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: [f32; 3],
    vec: Vec<f32>
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: [f32; 3]) -> Self {
        let vec = Vec::<f32>::new();

        for j in 0..2 {
            for i in j..3+j {
                let x_point = x;
                let y_point = y;

                if i % 2 != 0 {
                    y_point += height;
                }

                if i >= 2 {
                    x_point += width
                } 

                vec.push(x);
                vec.push(y);
                vec.push(0.5);
                vec.push(1.0);
                vec.push(color[0]);
                vec.push(color[1]);
                vec.push(color[2]);
            }
        }
        
        Self{x, y, width, height, color, vec}
    }
}
