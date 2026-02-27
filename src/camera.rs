use crate::prelude::*;

pub trait Camera: Sync + Send {
  // u, v is in [0, 1]
  fn get_ray(&self, u: Float, v: Float) -> Ray;
}

mod orthographic;
mod perspective;

pub use orthographic::OrthographicCamera;
pub use perspective::PerspectiveCamera;