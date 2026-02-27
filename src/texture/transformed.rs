use crate::prelude::*;

pub type Transformation = Box<dyn Fn(UV, &Point) -> (UV, Point) + Sync + Send>;

pub struct TransformedTexture {
  inner: Arc<dyn Texture>,
  transformation: Transformation,
}

impl TransformedTexture {
  pub fn new_with(texture: Arc<dyn Texture>, transformation: Transformation) -> Self {
    Self { inner: texture, transformation }
  }
  pub fn new<T>(texture: Arc<dyn Texture>, transformation: T) -> Self
  where T: Fn(UV, &Point) -> (UV, Point) + Sync + Send + 'static {
    Self { inner: texture, transformation: Box::new(transformation) }
  }
}

impl Texture for TransformedTexture {
  fn value(&self, uv: UV, point: &Point) -> ColorRgb {
    let (uv, point) = (self.transformation)(uv, point);
    self.inner.value(uv, &point)
  }
}