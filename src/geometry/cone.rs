/*
use crate::prelude::*;

/// apex: (0, 0, 1),
/// bottom circle: radius 1, center (0, 0, 0)
pub struct UnitCone {
  mat_bottom: Arc<dyn Material>,
  mat_side: Arc<dyn Material>,
}

impl UnitCone {
  pub fn new(mat_bottom: Arc<dyn Material>, mat_side: Arc<dyn Material>) -> Self {
    Self { mat_bottom, mat_side }
  }
}

impl Hittable for UnitCone {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

  }
  fn bounding_box(&self) -> Aabb {
    Aabb { min: Point::new(-1.0, -1.0, 0.0), max: Point::new(1.0, 1.0, 1.0) }
  }
}
*/