use crate::prelude::*;

pub struct ReinhardFilter;

impl Filter for ReinhardFilter {
  fn process(&self, film: &mut dyn Film) {
    for y in 0..film.height() {
      for x in 0..film.width() {
        let c = film.get_pixel(x, y);
        let mapped = ColorRgb::new(
          c.r / (1.0 + c.r),
          c.g / (1.0 + c.g),
          c.b / (1.0 + c.b),
        );
        film.set_pixel(x, y, mapped);
      }
    }
  }
}