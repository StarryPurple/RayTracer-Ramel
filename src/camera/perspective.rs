use crate::prelude::*;

pub struct PerspectiveCamera {
  origin: Point,
  lower_left_corner: Point,
  horizontal: Direction,
  vertical: Direction,
}

impl PerspectiveCamera {
  /// `vfov` shall be passed by degrees, like 45.0 instead of PI / 4.
  pub fn new(
    look_from: Point,
    look_at: Point,
    vup: Vec3d,
    vfov: Float,
    aspect_ratio: Float,
  ) -> Self {
    let theta = vfov.to_radians();
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = (look_from - look_at).normalize();
    let u = vup.cross(w).normalize();
    let v = w.cross(u);

    let origin = look_from;
    let horizontal = viewport_width * u;
    let vertical = viewport_height * v;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

    Self { origin, lower_left_corner, horizontal, vertical }
  }
}

impl Camera for PerspectiveCamera {
  fn get_ray(&self, u: Float, v: Float) -> Ray {
    Ray::new(
      self.origin,
      self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
    )
  }
}
