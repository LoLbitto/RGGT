use glutin::prelude::GlDisplay;
use ::gl::types::*; use std::ffi::CString;
use std::ops::Deref;
use std::ffi::CStr;

use crate::graphic::texture::Texture;
use crate::ui::text::Text;
use crate::ui::text::TextFabric;

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

    pub use Gles2 as Gl;
}

pub struct Renderer {
    program_solid: gl::types::GLuint,
    program_texture: gl::types::GLuint,
    program_text: gl::types::GLuint,
    
    // armazena o vbo e outros atributos do objeto
    vao_solid: gl::types::GLuint,
    vao_texture: gl::types::GLuint,
    vao_text: gl::types::GLuint,

    // armazena os vértices brutos de algo
    vbo_solid: gl::types::GLuint,
    vbo_texture: gl::types::GLuint,
    vbo_text: gl::types::GLuint,

    // comunicador com o opengl ou coisa parecida
    gl: gl::Gl,

    texture_map: Vec<u32>,

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

            let mut comp_stt: i32 = 1000;

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

            let pos_attrib_t = gl.GetAttribLocation(program_texture, b"position\0".as_ptr() as *const _);
            let color_attrib_t = gl.GetAttribLocation(program_texture, b"color\0".as_ptr() as *const _);
            let tex_attrib = gl.GetAttribLocation(program_texture, b"aTexCoord\0".as_ptr() as *const _);

            gl.VertexAttribPointer(
                pos_attrib_t as gl::types::GLuint,
                4,
                gl::FLOAT,
                0,
                9 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );

            gl.VertexAttribPointer(
                color_attrib_t as gl::types::GLuint,
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

            // CRIAÇÃO E MANEJAMENTO DE OBJETOS GRÁFICOS DE TEXTO
            let vertex_shader_text = create_shader(&gl, gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE_TEXT);
            let fragment_shader_text = create_shader(&gl, gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE_TEXT);

            let mut comp_stt = 0;

            gl.GetShaderiv(vertex_shader_text, gl::COMPILE_STATUS, &mut comp_stt);

            println!("compiled stt: {}", comp_stt);

            let program_text = gl.CreateProgram();

            gl.AttachShader(program_text, vertex_shader_text);
            gl.AttachShader(program_text, fragment_shader_text);

            gl.LinkProgram(program_text);

            gl.UseProgram(program_text);

            let projection = glm::ortho(0.0, 800.0, 0.0, 600.0, 1.0, -1.0);

            gl.UniformMatrix4fv(gl.GetUniformLocation(program_text, "projection".as_ptr() as *const i8), 1, gl::FALSE, glm::value_ptr(&projection).as_ptr());

            gl.DeleteShader(vertex_shader_text);
            gl.DeleteShader(fragment_shader_text);

            let mut vao_text = std::mem::zeroed();
            gl.GenVertexArrays(1, &mut vao_text);
            gl.BindVertexArray(vao_text);

                let mut vbo_text = std::mem::zeroed();
                gl.GenBuffers(1, &mut vbo_text);
                gl.BindBuffer(gl::ARRAY_BUFFER, vbo_text);

                // let pos_attrib = gl.GetAttribLocation(program_text, b"vertex\0".as_ptr() as *const _);
                            
                gl.VertexAttribPointer(
                    0 as gl::types::GLuint,
                    4,
                    gl::FLOAT,
                    0,
                    4 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                    std::ptr::null(),
                );

                gl.EnableVertexAttribArray(0 as gl::types::GLuint);

                // Definições finais

                gl.Enable(gl::DEPTH_TEST);
            
                let texture_map = vec![0];
                
                Self { program_solid, program_texture, program_text, vao_solid, vao_texture, vao_text, vbo_solid, vbo_texture, vbo_text, gl, texture_map}
            }
        }

        pub fn draw(&self) {
            self.draw_with_clear_color(0.1, 0.1, 0.1, 1.0)
        }

        pub fn update_solid(&mut self, vetores: &Vec<f32>) {        
            unsafe {

                self.gl.UseProgram(self.program_solid);
                
                self.gl.BindVertexArray(self.vao_solid);
                self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo_solid);

                self.gl.BufferData(
                    gl::ARRAY_BUFFER,
                    (vetores.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    vetores.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
            }
        }

        pub fn update_texture(&mut self, vetores: &Vec<f32>, textures: &mut Vec<*mut Texture>, texture_map: & Vec<u32>) { // Separando em 2 Métodos deixa mais organizado (eu acho)       
            unsafe {

                self.gl.UseProgram(self.program_texture);
                
                self.gl.BindVertexArray(self.vao_texture);
                self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo_texture);

                let textures: &mut Vec<*mut Texture> = textures.as_mut();

                if self.texture_map.len() < texture_map.len() {
                    self.texture_map = texture_map.clone(); 
                }

                for i in 0..textures.len() {
                    
                    let mut texture: &mut Texture = textures[i].as_mut().expect("Erro");

                    if !texture.has_id {

                        //println!("wow1: {}", *texture.get_id());
                        self.gl.GenTextures(1, texture.get_id());
                        self.gl.BindTexture(gl::TEXTURE_2D, *texture.get_id());
                        
                        self.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
                        self.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
                        self.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
                        self.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

                        self.gl.TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, texture.width, texture.height, 0, gl::RGB, gl::UNSIGNED_BYTE, texture.data.as_ptr() as *const _);
                        self.gl.GenerateMipmap(gl::TEXTURE_2D);

                        for j in 0..texture_map.len() {
                            if texture_map[j] == i as u32 {
                                self.texture_map[j] = *texture.get_id() as u32;
                            }
                        }
                    }
                }

                self.gl.BufferData(
                    gl::ARRAY_BUFFER,
                    (vetores.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    vetores.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
            }
        }

        pub fn clear_textures (&mut self) {
            if self.texture_map.len() > 0 {
                self.texture_map = Vec::<u32>::new();
                println!("eita abriu aqui heim");
                unsafe {
                    self.gl.UseProgram(self.program_texture);
                    self.gl.BindVertexArray(self.vao_texture);
                    self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo_texture);

                    self.gl.BufferData(
                        gl::ARRAY_BUFFER,
                        0 as gl::types::GLsizeiptr,
                        0 as *const _,
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
            
            self.gl.ClearColor(red, green, blue, alpha);
            
            unsafe{
                self.draw_solid();

                self.draw_texture();    
            }
        }
    }

    pub fn clear_buffers(&self) {
        unsafe {
            self.gl.UseProgram(self.program_solid);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);

            self.gl.UseProgram(self.program_texture);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        } 
    }

    pub unsafe fn draw_solid(&self) {
        self.gl.UseProgram(self.program_solid);
        self.gl.BindVertexArray(self.vao_solid);
        self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo_solid);

        let mut solid_size: GLint = 0;
        self.gl.GetBufferParameteriv(gl::ARRAY_BUFFER,gl::BUFFER_SIZE, &mut solid_size);
        solid_size = solid_size / 4;

        if solid_size > 1 {

            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.DrawArrays(gl::TRIANGLES, 0, solid_size);
        }
    }

    pub unsafe fn draw_texture(&self) {
        self.gl.UseProgram(self.program_texture);
        self.gl.BindVertexArray(self.vao_texture);
        self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo_texture);

        let mut texture_size: GLint = 0;
        self.gl.GetBufferParameteriv(gl::ARRAY_BUFFER,gl::BUFFER_SIZE, &mut texture_size);
        texture_size = texture_size / 4;

        if self.texture_map.len() > 1 {
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            for i in 0..texture_size / 27 {
                let inicio_triangulo = i as i32 * 3;
                let numero_vertices = 3;
                let textura = self.texture_map[i as usize];

                self.gl.BindTexture(gl::TEXTURE_2D, textura);

                self.gl.DrawArrays(gl::TRIANGLES, inicio_triangulo, numero_vertices);
            }
        }
    }

    pub unsafe fn draw_text(&mut self, texts: &mut Vec<Text>, text_fabric: &mut TextFabric) {
        
        self.clear_buffers();

        self.gl.UseProgram(self.program_text);

        self.gl.Uniform3f(self.gl.GetUniformLocation(self.program_text, "textColor".as_ptr() as *const i8), 1.0, 1.0, 1.0);

        self.gl.BindVertexArray(self.vao_text);
        self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo_text);

        self.gl.PixelStorei(gl::UNPACK_ALIGNMENT, 1);

        self.gl.Enable(gl::BLEND);
        self.gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);  

        for i in 0..texts.len() {
            let mut x = texts[i].x;
            let y = texts[i].y;

            let text = texts[i].text.clone();

            for char in text.chars() {
                let char_object = text_fabric.chars.get_mut(&char).unwrap();
                if char_object.tex_id == -1 {
                    let mut texture: u32 = 0;
                    self.gl.GenTextures(1, &mut texture);
                    self.gl.BindTexture(gl::TEXTURE_2D, texture);
                    
                    self.gl.TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RED as i32,
                        char_object.width,
                        char_object.height,
                        0,
                        gl::RED as u32,
                        gl::UNSIGNED_BYTE,
                        char_object.buffer.as_ptr() as *const _
                    );

                    self.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                    self.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                    self.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                    self.gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

                    char_object.tex_id = texture as i32;
                }

                let pos_x = x + char_object.bearing_x as f32 * texts[i].size;
                let pos_y = y + (char_object.height - char_object.bearing_y) as f32 * texts[i].size;

                let width = char_object.width as f32 * texts[i].size;
                let height = char_object.height as f32 * texts[i].size;

                let vertices: [f32; 24] = 
                [
                    pos_x,         pos_y + height, 0.0, 0.0,
                    pos_x,         pos_y,          0.0, 1.0,
                    pos_x + width, pos_y,          1.0, 1.0,

                    pos_x,         pos_y + height, 0.0, 0.0,
                    pos_x + width, pos_y,          1.0, 1.0,
                    pos_x + width, pos_y + height, 1.0, 0.0,
                ];

                //println!("x: {}, y: {}, w: {}, h: {}", pos_x, pos_y, width, height);

                self.gl.BindTexture(gl::TEXTURE_2D, char_object.tex_id as u32);
                
                println!("Char text: {}", char_object.tex_id);

                self.gl.BufferData(
                    gl::ARRAY_BUFFER,
                    (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                    vertices.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );

                self.gl.DrawArrays(gl::TRIANGLES, 0, vertices.len() as i32 * 4);

                x += (char_object.advance >> 6) as f32 + 10.0;

                println!("x: {}", x);
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
#version 100
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";

const VERTEX_SHADER_SOURCE_TEXTURE: &[u8] = b"
#version 330
precision mediump float;

attribute vec4 position;
attribute vec3 color;
attribute vec2 aTexCoord;

varying vec2 TexCoord;
varying vec3 v_color;

void main() {
    gl_Position = position;
    v_color = color;
    TexCoord = aTexCoord;
}
\0";

const FRAGMENT_SHADER_SOURCE_TEXTURE: &[u8] = b"
#version 330

varying vec3 v_color;
varying vec2 TexCoord;

uniform sampler2D ourTexture;

void main() {
    gl_FragColor = texture(ourTexture, TexCoord) * vec4(v_color, 1.0);
}
\0";

const VERTEX_SHADER_SOURCE_TEXT: &[u8] = b"
#version 330 core
layout (location = 0) in vec4 vertex; // <vec2 pos, vec2 tex>
out vec2 TexCoords;

uniform mat4 projection;

void main()
{
    gl_Position = projection * vec4(vertex.xy, 0.0, 1.0);
    TexCoords = vertex.zw;
} 
\0";

const FRAGMENT_SHADER_SOURCE_TEXT: &[u8] = b"
#version 330 core
in vec2 TexCoords;
out vec4 color;

uniform sampler2D text;
uniform vec3 textColor;

void main()
{    
    vec4 sampled = vec4(1.0, 1.0, 1.0, texture(text, TexCoords).r);
    color = vec4(textColor, 1.0) * sampled;
} 
\0";
