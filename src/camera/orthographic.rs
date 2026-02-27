use crate::camera::Camera;
use crate::prelude::*;

pub struct OrthographicCamera {
  origin: Point, // left-down corner
  horizonal: Vec3d,
  vertical: Vec3d,
  direction: Direction, // projection direction
}

impl OrthographicCamera {
  /// In local coordinate frame,
  /// assume the view ray (`look_at` - `look_from`) is from +z to -z,
  /// and `vup` is from -y to y.
  pub fn new(
    look_from: Point,
    look_at: Point,
    vup: Direction,
    height: Float,
    aspect_ratio: Float,
  ) -> Self {
    let width = aspect_ratio * height;

    let w = (look_from - look_at).normalize();
    let u = vup.cross(w).normalize();
    let v = w.cross(u);

    let horizonal = u * width;
    let vertical = v * height;
    let origin = look_from - (horizonal + vertical) / 2.0;

    Self {
      origin,
      horizonal,
      vertical,
      direction: -w,
    }
  }
}

impl Camera for OrthographicCamera {
  fn get_ray(&self, u: Float, v: Float) -> Ray {
    Ray::new(
      self.origin + u * self.horizonal + v * self.vertical,
      self.direction
    )
  }
}