use glutin::prelude::GlDisplay;
use ::gl::types::*;
use std::ffi::CString;
use std::ops::Deref;
use std::ffi::CStr;

use crate::graphic::texture::Texture;

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

    pub use Gles2 as Gl;
}

pub struct Renderer {
    program_solid: gl::types::GLuint,
    program_texture: gl::types::GLuint,
    
    // armazena o vbo e outros atributos do objeto
    vao_solid: gl::types::GLuint,
    vao_texture: gl::types::GLuint,

    // armazena os vértices brutos de algo
    vbo_solid: gl::types::GLuint,
    vbo_texture: gl::types::GLuint,

    // comunicador com o opengl ou coisa parecida
    gl: gl::Gl,

    vetores: Vec<f32>,

    textures: Vec<Texture>
}

impl Renderer {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Self {

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

            // CRIAÇÃO DE MANEJAMENTO DE OBJETOS COM CORES SÓLIDAS (SEM TEXTURA)

            let vertex_shader_solid = create_shader(&gl, gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
            let fragment_shader_solid = create_shader(&gl, gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

            let program_solid = gl.CreateProgram();

            gl.AttachShader(program_solid, vertex_shader_solid);
            gl.AttachShader(program_solid, fragment_shader_solid);

            gl.LinkProgram(program_solid);

            gl.UseProgram(program_solid);

            gl.DeleteShader(vertex_shader_solid);
            gl.DeleteShader(fragment_shader_solid);

            let mut vao_solid = std::mem::zeroed();
            gl.GenVertexArrays(1, &mut vao_solid);
            gl.BindVertexArray(vao_solid);

            let mut vbo_solid = std::mem::zeroed();
            gl.GenBuffers(1, &mut vbo_solid);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo_solid);

            let vetores = vec![0.0];

            let pos_attrib = gl.GetAttribLocation(program_solid, b"position\0".as_ptr() as *const _);
            let color_attrib = gl.GetAttribLocation(program_solid, b"color\0".as_ptr() as *const _);
            
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

            // CRIAÇÃO DE MANEJAMENTO DE OBJETOS COM TEXTURA

            let vertex_shader_texture = create_shader(&gl, gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE_TEXTURE);
            let fragment_shader_texture = create_shader(&gl, gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE_TEXTURE);

            let program_texture = gl.CreateProgram();

            gl.AttachShader(program_texture, vertex_shader_texture);
            gl.AttachShader(program_texture, fragment_shader_texture);

            gl.LinkProgram(program_texture);

            gl.UseProgram(program_texture);

            gl.DeleteShader(vertex_shader_texture);
            gl.DeleteShader(fragment_shader_texture);

            let mut vao_texture = std::mem::zeroed();
            gl.GenVertexArrays(1, &mut vao_texture);
            gl.BindVertexArray(vao_texture);

            let mut vbo_texture = std::mem::zeroed();
            gl.GenBuffers(1, &mut vbo_texture);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo_texture);

            let vetores_texture = vec![0.0];

            let pos_attrib = gl.GetAttribLocation(program_texture, b"position\0".as_ptr() as *const _);
            let color_attrib = gl.GetAttribLocation(program_texture, b"aColor\0".as_ptr() as *const _);
            let tex_attrib = gl.GetAttribLocation(program_texture, b"aTexCoord\0".as_ptr() as *const _);

            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                4,
                gl::FLOAT,
                0,
                9 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );

            gl.VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                9 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (4 * std::mem::size_of::<f32>()) as *const () as *const _,
            );

            gl.VertexAttribPointer(
                tex_attrib as gl::types::GLuint,
                2,
                gl::FLOAT,
                0,
                9 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (7 * std::mem::size_of::<f32>()) as *const () as *const _,
            );

            gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
            gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);
            gl.EnableVertexAttribArray(tex_attrib as gl::types::GLuint);

            gl.Enable(gl::DEPTH_TEST);

            let textures = Vec::<Texture>::new();
            
            Self { program_solid, program_texture, vao_solid, vao_texture, vbo_solid, vbo_texture, gl, vetores, textures}
        }
    }

    pub fn draw(&self) {
        self.draw_with_clear_color(0.1, 0.1, 0.1, 1.0)
    }

    pub fn update(&mut self, vetores: &Vec<f32>) {        
        unsafe {
            
            if self.vetores.len() >= vetores.len() { 
                for i in 0..self.vetores.len() {
                    if i < vetores.len() {
                        self.vetores[i] = vetores[i];
                    } else {
                        self.vetores[i] = 0.0;
                    }
                } // limpando o vetores
                self.gl.BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (self.vetores.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    self.vetores.as_ptr() as *const _,
                );
            } else {
                self.vetores = vetores.clone();
                self.gl.BufferData(
                    gl::ARRAY_BUFFER,
                    (vetores.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    self.vetores.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
            }
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
            
            self.gl.UseProgram(self.program_solid);
            
            self.gl.BindVertexArray(self.vao_solid);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo_solid);

            self.gl.ClearColor(red, green, blue, alpha);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            self.gl.DrawArrays(gl::TRIANGLES, 0, self.vetores.len() as i32);
            
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
            self.gl.DeleteProgram(self.program_solid);
            self.gl.DeleteBuffers(1, &self.vbo_solid);
            self.gl.DeleteVertexArrays(1, &self.vao_solid);
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
#version 330
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";

const VERTEX_SHADER_SOURCE_TEXTURE: &[u8] = b"
#version 330
precision mediump float;

layout (location = 0) in vec4 position;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;

out vec3 ourColor;
out vec2 TexCoord;

varying vec3 v_color;

void main() {
    gl_Position = position;
    ourColor = aColor;
    TexCoord = aTexCoord;
}
\0";

const FRAGMENT_SHADER_SOURCE_TEXTURE: &[u8] = b"
#version 330
out vec4 FragColor

in vec3 ourColor;
in vec2 TexCoord;

uniform sampler2D ourTexture;

void main() {
    FragColor = texture(ourTexture, TexCoord) * vec4(ourColor, 1.0);
}
\0";


