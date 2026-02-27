use crate::prelude::*;

pub trait Texture: Send + Sync {
  fn value(&self, uv: UV, point: &Point) -> ColorRgb;
}

pub mod solid_color;
pub mod noise;
pub mod image;
pub use image::ImageTexture;
pub mod checker;
pub mod transformed;

pub use solid_color::SolidColorTexture;
pub use noise::PerlinNoiseTexture;
pub use checker::{SpatialCheckerTexture, UVCheckerTexture};
pub use transformed::TransformedTexture;