use image::error::ImageResult;
use image::ImageReader;
use image::DynamicImage;
use image::ColorType;

use crate::resources::file_manager::assets;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub nr_channels: u8,
    pub data: Vec<u8>
}

impl Texture {
    pub fn new(texture_name: &str) -> Self {
        let image_result = assets::get_image(texture_name);
        let mut data = Vec::<u8>::new();
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut nr_channels: u8 = 0;

        match image_result {
            Ok(image) => {
                data = image.as_bytes().to_vec();
                width = image.width();
                height = image.height();
                nr_channels = image.color().channel_count();
            },

            Err(e) => {

            }
        }

        Self {width, height, nr_channels, data}
    }
}
