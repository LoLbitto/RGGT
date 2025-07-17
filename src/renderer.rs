use glutin::prelude::GlDisplay;
use ::gl::types::*;
use std::ffi::CString;
use std::ops::Deref;
use std::ffi::CStr;

use crate::object::visualrep::Visual;

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

    pub use Gles2 as Gl;
}

pub struct Renderer {
    program: gl::types::GLuint,
    
    // armazena o vbo e outros atributos do objeto
    vao: gl::types::GLuint,

    // armazena os vértices brutos de algo
    vbo: gl::types::GLuint,

    // comunicador com o opengl ou coisa parecida
    gl: gl::Gl,

    objetos: Vec<Visual>,
}

impl Renderer {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Self {
        let mut objetos = Vec::new();
        let visual1 = Visual::new(vec![ 0.0,  0.5,  0.0,  1.0, 0.0, 0.0,
                                       -0.5, -0.5,  0.5,  0.0, 0.0, 1.0,
                                        0.5, -0.5,  0.5,  0.0, 1.0, 0.0,

                                        0.0,  0.5,  0.0,  1.0, 0.0, 0.0,
                                       -0.5, -0.5, -0.5,  0.0, 1.0, 0.0,
                                        0.5, -0.5, -0.5,  0.0, 0.0, 1.0,

                                        0.0,  0.5,  0.0,  1.0, 0.0, 0.0,
                                       -0.5, -0.5, -0.5,  0.0, 1.0, 0.0,
                                       -0.5, -0.5,  0.5,  0.0, 0.0, 1.0,

                                        0.0,  0.5,  0.0,  1.0, 0.0, 0.0,
                                        0.5, -0.5, -0.5,  0.0, 0.0, 1.0,
                                        0.5, -0.5,  0.5,  0.0, 1.0, 0.0,
                                      ],
                                 gl::TRIANGLES);
        
        objetos.push(visual1);

        unsafe {
            let gl = gl::Gl::load_with(|symbol| {
                let symbol = CString::new(symbol).unwrap();
                gl_display.get_proc_address(symbol.as_c_str()).cast()
            });

            if let Some(renderer) = get_gl_string(&gl, gl::RENDERER) {
                println!("Running on {}", renderer.to_string_lossy());
            }
            if let Some(version) = get_gl_string(&gl, gl::VERSION) {
                println!("OpenGL Version {}", version.to_string_lossy());
            }

            if let Some(shaders_version) = get_gl_string(&gl, gl::SHADING_LANGUAGE_VERSION) {
                println!("Shaders version on {}", shaders_version.to_string_lossy());
            }

            let vertex_shader = create_shader(&gl, gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
            let fragment_shader = create_shader(&gl, gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

            let program = gl.CreateProgram();

            gl.AttachShader(program, vertex_shader);
            gl.AttachShader(program, fragment_shader);

            gl.LinkProgram(program);

            gl.UseProgram(program);

            gl.DeleteShader(vertex_shader);
            gl.DeleteShader(fragment_shader);

            let mut vao = std::mem::zeroed();
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);

            let mut vbo = std::mem::zeroed();
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);

            let mut len = 0;
            let mut vetores = Vec::new();

            for i in 0..objetos.len() {
                len += objetos[i].vertex.len();
                vetores.extend(objetos[i].vertex.iter().cloned());
            }

            gl.BufferData(
                gl::ARRAY_BUFFER,
                (len * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vetores.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let pos_attrib = gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
            let color_attrib = gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl.VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (3 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
            gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);

            Self { program, vao, vbo, gl, objetos }
        }
    }

    pub fn draw(&self) {
        self.draw_with_clear_color(0.1, 0.1, 0.1, 1.0)
    }

    pub fn update(&mut self, mut x: f32) {
        for i in 0..self.objetos.len() {
            self.objetos[i].rotateX(1.0);
        }
        
        unsafe {
        self.gl.BufferSubData(
            gl::ARRAY_BUFFER,
            0,
            (self.objetos[0].vertex.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            self.objetos[0].vertex.as_ptr() as *const _,
            );
        }
        println!("tentou ne {x}");
    }

    pub fn draw_with_clear_color(
        &self,
        red: GLfloat,
        green: GLfloat,
        blue: GLfloat,
        alpha: GLfloat,
    ) {
        unsafe {
            self.gl.UseProgram(self.program);

            self.gl.BindVertexArray(self.vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.gl.ClearColor(red, green, blue, alpha);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            
            let mut lastSize = 0;

            for i in 0..self.objetos.len() {

                let size = self.objetos[i].vertex.len() as i32 / 6; 
                self.gl.DrawArrays(self.objetos[i].tipo, lastSize, size as i32);
                lastSize = size ;

            }
            
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }
}

impl Deref for Renderer {
    type Target = gl::Gl;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.program);
            self.gl.DeleteBuffers(1, &self.vbo);
            self.gl.DeleteVertexArrays(1, &self.vao);
        }
    }
}

unsafe fn create_shader(
    gl: &gl::Gl,
    shader: gl::types::GLenum,
    source: &[u8],
) -> gl::types::GLuint {
    let shader = gl.CreateShader(shader);
    gl.ShaderSource(shader, 1, [source.as_ptr().cast()].as_ptr(), std::ptr::null());
    gl.CompileShader(shader);
    shader
}

fn get_gl_string(gl: &gl::Gl, variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl.GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}

const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

attribute vec3 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = vec4(position, 1.0);
    v_color = color;
}
\0";

const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";


