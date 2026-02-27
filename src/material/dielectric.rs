use crate::prelude::*;

pub struct Dielectric {
  pub ir: Float,
  pub albedo: Arc<dyn Texture>,
}