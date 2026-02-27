use crate::prelude::*;

pub trait Film: Send + Sync {
  fn width(&self) -> u32;
  fn height(&self) -> u32;
  fn aspect_ratio(&self) -> Float {
    self.width() as Float / self.height() as Float
  }

  fn set_pixel(&mut self, x: u32, y: u32, color: ColorRgb);
  fn get_pixel(&self, x: u32, y: u32) -> ColorRgb;
  
  fn to_image_linear(&self) -> image::RgbaImage;
  fn to_image_srgb(&self) -> image::RgbaImage;
}

mod simple;

pub use simple::SimpleFilm;
