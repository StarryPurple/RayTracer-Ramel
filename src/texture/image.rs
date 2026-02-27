use crate::prelude::*;
use image::GenericImageView;

pub struct ImageTexture {
  image: image::DynamicImage,
  is_srgb: bool,
}

impl ImageTexture {
  pub fn new(path: &str, is_srgb: bool) -> Self {
    let image = image::open(path).expect("Failed to load texture image");
    Self { image, is_srgb }
  }
}

impl Texture for ImageTexture {
  fn value(&self, uv: UV, _point: &Point) -> ColorRgb {
    let (width, height) = self.image.dimensions();
    let i = (uv.u * width as Float) as u32;
    let j = ((1.0 - uv.v) * height as Float) as u32;
    let i = if i < width { i } else { width - 1 };
    let j = if j < height { j } else { height - 1 };
    let pixel = self.image.get_pixel(i, j);
    let color = ColorRgb::from_rgba(pixel);
    if self.is_srgb {
      color.to_linear()
    } else {
      color
    }
  }
}
