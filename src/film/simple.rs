use crate::prelude::*;

pub struct SimpleFilm {
  width: u32,
  height: u32,
  buffer: Vec<ColorRgb>,
}

impl SimpleFilm {
  #[inline]
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      width,
      height,
      buffer: vec![ColorRgb::BLACK; (width * height) as usize],
    }
  }
}

impl Film for SimpleFilm {
  #[inline]
  fn width(&self) -> u32 {
    self.width
  }
  #[inline]
  fn height(&self) -> u32 {
    self.height
  }
  #[inline]
  fn set_pixel(&mut self, x: u32, y: u32, color: ColorRgb) {
    self.buffer[(y * self.width + x) as usize] = color;
  }
  #[inline]
  fn get_pixel(&self, x: u32, y: u32) -> ColorRgb {
    self.buffer[(y * self.width + x) as usize]
  }
  fn to_image_linear(&self) -> image::RgbaImage {
    let mut image = image::RgbaImage::new(self.width, self.height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
      *pixel = self.buffer[(y * self.width + x) as usize].into_rgba();
    }
    image
  }
  fn to_image_srgb(&self) -> image::RgbaImage {
    let mut image = image::RgbaImage::new(self.width, self.height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
      *pixel = self.buffer[(y * self.width + x) as usize].to_gamma().into_rgba();
    }
    image
  }
}
