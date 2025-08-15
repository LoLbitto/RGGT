use image::error::ImageResult;
use image::ImageReader;
use image::DynamicImage;
use image::ColorType;

use crate::resources::file_manager::assets;

pub struct Texture {
    pub width: i32,
    pub height: i32,
    pub nr_channels: u8,
    pub data: Vec<u8>,
    
    gl_id: u32,
    pub has_id: bool,
}

impl Texture {
    pub fn new(texture_name: &str) -> Self {
        let image_result = assets::get_image(texture_name);
        let mut data = Vec::<u8>::new();
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut nr_channels: u8 = 0;

        let gl_id = 0;
        let has_id = false;

        match image_result {
            Ok(image) => {
                data = image.as_bytes().to_vec();
                width = image.width() as i32;
                height = image.height() as i32;
                nr_channels = image.color().channel_count();

                println!("w: {}, h: {}, sla: {}", width, height, data[9]);
            },

            Err(e) => {

            }
        }

        Self {width, height, nr_channels, data, gl_id, has_id}
    }

    pub fn get_id(&mut self) -> &mut u32 {
        self.has_id = true;
        &mut self.gl_id
    }
}
