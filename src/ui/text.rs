extern crate freetype;

use std::collections::HashMap;

use freetype::face::Face;
use freetype::face::LoadFlag;
use freetype::glyph_slot::GlyphSlot;

use crate::resources::file_manager::assets;

pub struct Text<'a> {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub char_array: &'a mut HashMap<char, Char>
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
    chars: HashMap<char, Char>
}

impl TextFabric {
    pub fn new(font_name: String) -> Self {
        let font = assets::get_font(&font_name);
        let mut chars = HashMap::<char, Char>::new();

        for i in 0..128 { // carrega os 128 caracteres da tabela ascii
            
            font.load_char(i, LoadFlag::RENDER).unwrap();

            let glyph = font.glyph();

            let char = Self::get_char_from_glyph(glyph);

            chars.insert(i as u8 as char, char);
        }

        Self {font, chars}
    }

    pub fn gen_text(&mut self, text: String, x: f32, y: f32) -> Text {

        for caractere in text.chars() {

            if !self.chars.contains_key(&caractere) {
                self.create_char(caractere);
            } 
        } 

        Text{x, y, text, char_array: &mut self.chars}
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
