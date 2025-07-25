use glutin::prelude::GlDisplay;
use ::gl::types::*;
use std::ffi::CString;
use std::ops::Deref;
use std::ffi::CStr;

use crate::entity::object::Object;
use crate::entity::player::Player;

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

    objetos: Vec<Object>,

    player: *const Player,

    vetores: Vec<f32>,
}

impl Renderer {
    pub fn new<D: GlDisplay>(gl_display: &D, player: *const Player) -> Self {
        
        let mut objetos = Vec::new();
        let objeto = Object::new();

        objetos.push(objeto);

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
            
            let mut vetores = Vec::<f32>::new();

            for i in 0..objetos.len() {
             
                if objetos[i].verify_on_screen((*player).position, (*player).mira) {
                    let grap_rep = objetos[i].visual.as_ref().unwrap();
                    vetores.extend(grap_rep.vertex.iter().cloned());
                }
            }

            gl.BufferData(
                gl::ARRAY_BUFFER,
                (vetores.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vetores.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let pos_attrib = gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
            let color_attrib = gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                4,
                gl::FLOAT,
                0,
                7 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl.VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                7 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (4 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
            gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);


            gl.Enable(gl::DEPTH_TEST);
            
            Self { program, vao, vbo, gl, objetos, player, vetores}
        }
    }

    pub fn draw(&self) {
        self.draw_with_clear_color(0.1, 0.1, 0.1, 1.0)
    }

    pub fn update(&mut self) {        
        unsafe {
            let mut vetores = Vec::<f32>::new();

            for i in 0..self.objetos.len() {
             
                if self.objetos[i].verify_on_screen((*self.player).position, (*self.player).mira) {
                    let grap_rep = self.objetos[i].visual.as_ref().unwrap();
                    vetores.extend(grap_rep.vertex.iter().cloned());
                }
            }

            if vetores.len() == 0 {
                vetores = Vec::<f32>::new();
                for i in 0..self.vetores.len() {
                    self.vetores[i] = 0.0;
                }
                vetores = self.vetores.clone();
            } else {
                self.vetores = vetores.clone();
            }

            self.gl.BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (vetores.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vetores.as_ptr() as *const _,
            );
        }
    }

    pub fn draw_with_clear_color(
        &self,
        red: GLfloat,
        green: GLfloat,
        blue: GLfloat,
        alpha: GLfloat,
    ) {
        unsafe {
            self.gl.Clear(gl::DEPTH_BUFFER_BIT);
            self.gl.UseProgram(self.program);

            self.gl.BindVertexArray(self.vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.gl.ClearColor(red, green, blue, alpha);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            self.gl.DrawArrays(gl::TRIANGLES, 0, 106);
            
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
) -> gl::types::GLuint { unsafe {
    let shader = gl.CreateShader(shader);
    gl.ShaderSource(shader, 1, [source.as_ptr().cast()].as_ptr(), std::ptr::null());
    gl.CompileShader(shader);
    shader
}}

fn get_gl_string(gl: &gl::Gl, variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl.GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}

const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

attribute vec4 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = position;
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


