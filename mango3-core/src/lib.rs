use std::{fs, io};

use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut, text_size};
use imageproc::rect::Rect;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[cfg(test)]
mod test_utils;

pub mod config;
pub mod constants;
pub mod enums;
pub mod jobs;
pub mod locales;
pub mod models;
pub mod pagination;
pub mod validator;

use config::{DATABASE_CONFIG, MISC_CONFIG};
use jobs::Jobs;

type DBPool = PgPool;

async fn setup_db_pool() -> DBPool {
    PgPoolOptions::new()
        .max_connections(DATABASE_CONFIG.max_connections as u32)
        .connect(&DATABASE_CONFIG.url)
        .await
        .expect("Failed to create DB pool.")
}

#[derive(Clone)]
pub struct CoreContext {
    db_pool: DBPool,
    pub jobs: Jobs,
}

impl CoreContext {
    pub async fn setup() -> Self {
        Self {
            db_pool: setup_db_pool().await,
            jobs: Jobs::setup().await,
        }
    }
}

pub fn text_icon(text: String, size: u16) -> io::Result<Vec<u8>> {
    let dir_path = format!("{}/text-icons/{}", MISC_CONFIG.storage_path, text);
    let file_path = format!("{}/{}x{}.png", dir_path, size, size);
    let size = size as u32;

    if !fs::exists(&file_path).unwrap_or_default() {
        let mut rgb_image = RgbImage::new(size, size);

        draw_filled_rect_mut(
            &mut rgb_image,
            Rect::at(0, 0).of_size(size, size),
            Rgb([111u8, 111u8, 111u8]),
        );

        let font_file = fs::read(&MISC_CONFIG.font_path).expect("Could not read font file");
        let font = FontRef::try_from_slice(&font_file).expect("Could not get font");
        let scale = PxScale::from(size as f32 / 1.7);
        let (text_width, _) = text_size(scale, &font, &text);
        let x = ((size - text_width) / 2) as i32;
        let y = (size as f32 / 4.6) as i32;

        draw_text_mut(&mut rgb_image, Rgb([225u8, 225u8, 225u8]), x, y, scale, &font, &text);

        let _ = fs::create_dir_all(dir_path);
        let _ = rgb_image.save(file_path.clone());
    }

    fs::read(file_path)
}
