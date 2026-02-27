use crate::prelude::*;

pub trait Filter {
  fn process(&self, film: &mut dyn Film);
}

pub struct FilterList {
  filters: Vec<Arc<dyn Filter>>,
}

impl FilterList {
  pub fn new() -> Self {
    FilterList { filters: vec![] }
  }
  pub fn add_filter(&mut self, filter: Arc<dyn Filter>) {
    self.filters.push(filter);
  }
  pub fn filters(&self) -> &Vec<Arc<dyn Filter>> {
    &self.filters
  }
  pub fn filters_mut(&mut self) -> &mut Vec<Arc<dyn Filter>> {
    &mut self.filters
  }
  pub fn process(&self, film: &mut dyn Film) {
    for filter in &self.filters {
      filter.process(film);
    }
  }
}

mod gamma;
mod exposure;
mod aces;
mod vignette;
mod reinhard;

pub use gamma::GammaFilter;
pub use exposure::ExposureFilter;
pub use aces::AcesFilmicFilter;
pub use vignette::VignetteFilter;
pub use reinhard::ReinhardFilter;