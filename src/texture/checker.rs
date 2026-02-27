use crate::prelude::*;

pub struct SpatialCheckerTexture {
  scale_x: Float,
  scale_y: Float,
  scale_z: Float,
  even: Arc<dyn Texture>,
  odd: Arc<dyn Texture>,
}

impl SpatialCheckerTexture {
  pub fn new(
    scale_x: Float,
    scale_y: Float,
    scale_z: Float,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
  ) -> Self {
    Self {
      scale_x,
      scale_y,
      scale_z,
      even,
      odd,
    }
  }

  pub fn from_colors(
    scale_x: Float,
    scale_y: Float,
    scale_z: Float,
    c1: ColorRgb,
    c2: ColorRgb,
  ) -> Self {
    Self::new(
      scale_x,
      scale_y,
      scale_z,
      Arc::new(texture::SolidColorTexture::new(c1)),
      Arc::new(texture::SolidColorTexture::new(c2)),
    )
  }
}

impl Texture for SpatialCheckerTexture {
  fn value(&self, uv: UV, point: &Point) -> ColorRgb {
    let x = (point.x * self.scale_x).floor() as i32;
    let y = (point.y * self.scale_y).floor() as i32;
    let z = (point.z * self.scale_z).floor() as i32;
    if x + y + z % 2 == 0 {
      self.even.value(uv, point)
    } else {
      self.odd.value(uv, point)
    }
  }
}

pub struct UVCheckerTexture {
  scale_u: Float,
  scale_v: Float,
  even: Arc<dyn Texture>,
  odd: Arc<dyn Texture>,
}

impl UVCheckerTexture {
  pub fn new(
    scale_u: Float,
    scale_v: Float,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
  ) -> Self {
    Self { scale_u, scale_v, even, odd }
  }
  
  pub fn new_arc(
    scale_u: Float,
    scale_v: Float,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
  ) -> Arc<Self> {
    Arc::new(Self { scale_u, scale_v, even, odd })
  }

  pub fn from_colors(scale_u: Float, scale_v: Float, c1: ColorRgb, c2: ColorRgb) -> Self {
    Self::new(
      scale_u,
      scale_v,
      Arc::new(texture::SolidColorTexture::new(c1)),
      Arc::new(texture::SolidColorTexture::new(c2)),
    )
  }
}

impl Texture for UVCheckerTexture {
  fn value(&self, uv: UV, point: &Point) -> ColorRgb {
    let u_grid = (uv.u * self.scale_u).floor() as i32;
    let v_grid = (uv.v * self.scale_v).floor() as i32;

    if (u_grid + v_grid) % 2 == 0 {
      self.even.value(uv, point)
    } else {
      self.odd.value(uv, point)
    }
  }
}
