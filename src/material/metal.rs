use crate::prelude::*;

pub struct Metal {
  albedo: ColorRgb,
  fuzz: f32,
}
impl Metal {
  pub fn new(albedo: ColorRgb, fuzz: f32) -> Self {
    Self { albedo, fuzz }
  }
  pub fn new_arc(albedo: ColorRgb, fuzz: f32) -> Arc<Self> {
    Arc::new(Self { albedo, fuzz })
  }
}
impl Material for Metal {
  fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(ColorRgb, Ray)> {
    let reflected = ray_in.reflect(record.hit_t, record.unit_normal).direction.normalize() + self.fuzz * Point::random_unit();
    let scattered = Ray::new(record.point, reflected);
    if scattered.direction.is_facing(record.unit_normal) {
      Some((self.albedo, scattered))
    } else {
      None
    }
  }
}