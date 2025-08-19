extern crate freetype;

use std::collections::HashMap;

use freetype::face::Face;
use freetype::face::LoadFlag;

use crate::resources::file_manager::assets;

pub struct Text<'a> {
    pub chars: Vec<&'a Char>,
}

pub struct Char {
    tex_id: i32,
    width: i32,
    height: i32,
    bearing_x: i32,
    bearing_y: i32,
    buffer: Vec<u8>
}

pub struct TextFabric {
    font: Face,
    chars: HashMap<char, Char>
}

impl TextFabric {
    pub fn new(font_name: &str) -> Self {
        let font = assets::get_font(font_name);
        let mut chars = HashMap::<char, Char>::new();

        for i in 0..128 { // carrega os 128 caracteres da tabela ascii
            
            font.load_char(i, LoadFlag::RENDER).unwrap();

            let glyph = font.glyph();

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

            chars.insert(i as u8 as char, char);
        }

        Self {font, chars}
    }
}
