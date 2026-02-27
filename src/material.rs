use crate::prelude::*;

pub trait Material: Send + Sync {
  fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(ColorRgb, Ray)>;
  fn emitted(&self, _uv: UV, _p: Point) -> ColorRgb {
    ColorRgb::BLACK
  }
}

pub fn convert_material(_t_mat: tobj::Material) -> Arc<dyn Material> {
  unimplemented!()
}

mod lambertian;
mod diffusion_light;
mod metal;
mod dielectric;

pub use lambertian::Lambertian;
pub use diffusion_light::DiffusionLight;
pub use metal::Metal;
pub use dielectric::Dielectric;