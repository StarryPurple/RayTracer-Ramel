use crate::prelude::*;
use crate::texture::Texture;

pub struct SolidColorTexture {
  color: ColorRgb,
}

impl SolidColorTexture {
  pub fn new(color: ColorRgb) -> Self {
    Self { color }
  }
  pub fn new_arc(color: ColorRgb) -> Arc<Self> {
    Arc::new(Self { color })
  }
}

impl Texture for SolidColorTexture {
  fn value(&self, _uv: UV, _point: &Point) -> ColorRgb {
    self.color
  }
}