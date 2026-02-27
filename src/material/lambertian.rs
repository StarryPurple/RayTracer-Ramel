use crate::material::Material;
use crate::prelude::*;

pub struct Lambertian {
  pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
  #[inline]
  pub fn new(albedo: Arc<dyn Texture>) -> Self {
    Self { albedo }
  }
  pub fn new_arc(albedo: Arc<dyn Texture>) -> Arc<Self> {
    Arc::new(Self { albedo })
  }
}

impl Material for Lambertian {
  fn scatter(&self, _ray_in: &Ray, record: &HitRecord) -> Option<(ColorRgb, Ray)> {
    let mut direction = record.unit_normal + Vec3d::random_unit();
    if direction.near_zero() {
      direction = record.unit_normal;
    }
    let scattered = Ray::new(record.point, direction);
    let attenuation = self.albedo.value(record.mat_uv, &record.point);
    Some((attenuation, scattered))
  }
}
