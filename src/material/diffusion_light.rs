use crate::prelude::*;

pub struct DiffusionLight {
  tex_emit: Arc<dyn Texture>,
}
impl DiffusionLight {
  pub fn new(tex_emit: Arc<dyn Texture>) -> Self {
    Self { tex_emit }
  }
  pub fn from_color(c: ColorRgb) -> Self {
    Self { tex_emit: Arc::new(texture::SolidColorTexture::new(c)) }
  }
  pub fn new_arc(tex_emit: Arc<dyn Texture>) -> Arc<Self> {
    Arc::new(Self { tex_emit })
  }
  pub fn arc_from_color(c: ColorRgb) -> Arc<Self> {
    Arc::new(Self::from_color(c))
  }
}
impl Material for DiffusionLight {
  fn scatter(&self, _ray_in: &Ray, _record: &HitRecord) -> Option<(ColorRgb, Ray)> {
    None
  }
  fn emitted(&self, uv: UV, p: Point) -> ColorRgb {
    self.tex_emit.value(uv, &p)
  }
}