/*
use crate::prelude::*;

/// top circle: radius 1, center (0, 0, 1)
/// bottom circle: radius 1, center (0, 0, 0)
pub struct UnitCylinder {
  mat_bottom: Arc<dyn Material>,
  mat_top: Arc<dyn Material>,
  mat_side: Arc<dyn Material>,
}

impl UnitCylinder {
  pub fn new(mat_bottom: Arc<dyn Material>, mat_side: Arc<dyn Material>, mat_top: Arc<dyn Material>) -> Self {
    Self { mat_bottom, mat_side, mat_top }
  }
}

impl Hittable for UnitCylinder {
  fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

  }
  fn bounding_box(&self) -> Aabb {
    Aabb { min: Point::new(-1.0, -1.0, 0.0), max: Point::new(1.0, 1.0, 1.0) }
  }
}
*/