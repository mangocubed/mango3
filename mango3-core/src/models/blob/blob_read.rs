use std::fs;
use std::path::Path;

use image::metadata::Orientation;
use image::{DynamicImage, ImageDecoder, ImageReader};

use crate::config::MISC_CONFIG;

use super::Blob;

impl Blob {
    pub fn read(&self, width: Option<u16>, height: Option<u16>, fill: Option<bool>) -> Option<Vec<u8>> {
        if width.is_some() && height.is_some() {
            let width = width.unwrap();
            let height = height.unwrap();
            let fill = fill.unwrap_or(false);

            let variant_path = self.image_variant_path(width, height, fill);

            if !Path::new(&variant_path).exists() {
                let mut image_decoder = ImageReader::open(self.default_path())
                    .expect("Could not get image")
                    .into_decoder()
                    .expect("Could not convert image into decoder");
                let orientation = image_decoder.orientation().unwrap_or(Orientation::NoTransforms);
                let mut dynamic_image = DynamicImage::from_decoder(image_decoder).expect("Could not get dynamic image");

                dynamic_image.apply_orientation(orientation);

                dynamic_image = if fill {
                    dynamic_image.resize_to_fill(width as u32, height as u32, MISC_CONFIG.image_ops_filter_type())
                } else {
                    dynamic_image.resize(width as u32, height as u32, MISC_CONFIG.image_ops_filter_type())
                };

                dynamic_image.save(variant_path.clone()).unwrap();
            }

            return fs::read(variant_path).ok();
        }

        fs::read(self.default_path()).ok()
    }
}
