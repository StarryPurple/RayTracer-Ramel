use crate::prelude::*;

pub struct GammaFilter {
  gamma: Float,
}

impl GammaFilter {
  pub fn new(gamma: Float) -> Self {
    Self { gamma }
  }
  pub fn gamma(&self) -> Float {
    self.gamma
  }
}

impl Filter for GammaFilter {
  fn process(&self, film: &mut dyn Film) {
    let inv = 1.0 / self.gamma;
    let width = film.width();
    let height = film.height();
    for y in 0..height {
      for x in 0..width {
        let color = film.get_pixel(x, y);

        let corrected_color = ColorRgb::new(
          color.r.powf(inv),
          color.g.powf(inv),
          color.b.powf(inv),
        );

        film.set_pixel(x, y, corrected_color);
      }
    }
  }
}