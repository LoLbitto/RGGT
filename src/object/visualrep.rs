use ::gl::types::*;

pub struct Visual {
    pub vertex: Vec<f32>,
    pub tipo:  GLenum,
}

impl Visual {
    pub fn new (vertex: Vec<f32>, tipo: GLenum) -> Self {
        Self {vertex, tipo}
    }
}
