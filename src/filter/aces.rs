use crate::prelude::*;

pub struct AcesFilmicFilter;

impl Filter for AcesFilmicFilter {
  fn process(&self, film: &mut dyn Film) {
    for y in 0..film.height() {
      for x in 0..film.width() {
        let color = film.get_pixel(x, y);
        
        let a = 2.51;
        let b = 0.03;
        let c = 2.43;
        let d = 0.59;
        let e = 0.14;

        let map = |x: Float| -> Float {
          ((x * (a * x + b)) / (x * (c * x + d) + e)).clamp(0.0, 1.0)
        };

        let mapped = ColorRgb::new(map(color.r), map(color.g), map(color.b));
        film.set_pixel(x, y, mapped);
      }
    }
  }
}