
use crate::prelude::*;

pub struct RenderConfig {
  pub film: Arc<Mutex<dyn Film>>,
  pub camera: Arc<dyn Camera>,
  pub filters: Arc<FilterList>,
  pub world: Arc<World>,
}

pub trait Renderer {
  fn render(&self, config: RenderConfig);
}

mod simple;

pub use simple::SimpleRenderer;