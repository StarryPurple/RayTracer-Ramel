use crate::prelude::*;

pub struct VignetteFilter {
  pub intensity: Float,
}

impl VignetteFilter {
  pub fn new(intensity: Float) -> Self {
    Self { intensity }
  }
}

impl Filter for VignetteFilter {
  fn process(&self, film: &mut dyn Film) {
    let (w, h) = (film.width() as Float, film.height() as Float);
    for y in 0..film.height() {
      for x in 0..film.width() {
        let u = (x as Float + 0.5) / w - 0.5;
        let v = (y as Float + 0.5) / h - 0.5;
        let dist = (u * u + v * v).sqrt();
        let factor = 1.0 - dist * self.intensity;

        let color = film.get_pixel(x, y);
        film.set_pixel(x, y, color * factor.max(0.0));
      }
    }
  }
}