extern crate freetype;

use std::collections::HashMap;

use freetype::face::Face;
use freetype::face::LoadFlag;
use freetype::glyph_slot::GlyphSlot;

use crate::resources::file_manager::assets;

pub struct Text {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub font: String,
}

impl Text {
    pub fn new(text: String, x: f32, y:f32, size:f32, font: String) -> Self {
        Self{text, x, y, size, font}
    }
}

pub struct Char {
    pub tex_id: i32,
    pub width: i32,
    pub height: i32,
    pub bearing_x: i32,
    pub bearing_y: i32,
    pub buffer: Vec<u8>
}

pub struct TextFabric {
    font: Face,
    pub font_name: String,
    pub chars: HashMap<char, Char>
}

impl TextFabric {
    pub fn new(font_name: String) -> Self {
        let font = assets::get_font(&font_name);
        let mut chars = HashMap::<char, Char>::new();

       for i in 0..128 { // carrega os 128 caracteres da tabela ascii

            font.set_char_size(40 * 64, 0, 50, 0).unwrap();

            font.load_char(i, LoadFlag::RENDER).unwrap();

            let glyph = font.glyph();

            let char = Self::get_char_from_glyph(glyph);

            chars.insert(i as u8 as char, char);
        }

        Self {font, font_name, chars}
    }

    pub fn create_char(&mut self, char: char) {
        self.font.load_char(char as usize, LoadFlag::RENDER).unwrap();
        
        let glyph = self.font.glyph();
        
        let char_loaded = Self::get_char_from_glyph(glyph);
        
        self.chars.insert(char, char_loaded);
    }

    fn get_char_from_glyph(glyph: &GlyphSlot) -> Char {
        
        let tex_id = -1; // Não carregada ainda
        let width = glyph.bitmap().width();
        let height = glyph.bitmap().rows();
        let bearing_x = glyph.bitmap_left();
        let bearing_y = glyph.bitmap_top();
        let buffer = glyph.bitmap().buffer().to_vec();

        let char = Char {
            tex_id,
            width,
            height,
            bearing_x,
            bearing_y,
            buffer
        };
        
        char
    }
}
