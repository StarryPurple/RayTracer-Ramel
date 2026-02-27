use crate::prelude::*;

pub struct ExposureFilter {
  pub exposure: Float,
}

impl ExposureFilter {
  pub fn new(exposure: Float) -> ExposureFilter {
    ExposureFilter { exposure }
  }
}

impl Filter for ExposureFilter {
  fn process(&self, film: &mut dyn Film) {
    let factor = 2.0_f32.powf(self.exposure);
    for y in 0..film.height() {
      for x in 0..film.width() {
        let color = film.get_pixel(x, y);
        film.set_pixel(x, y, color * factor);
      }
    }
  }
}