use crate::prelude::*;

/// UnitQuad is on XY plane.
/// Interpreted as [-0.5, 0.5] x [-0.5, 0.5] x {0}.
/// the outward normal is Z+, or (0, 0, 1).
pub struct UnitQuad {
  mat: Arc<dyn Material>,
}

impl UnitQuad {
  pub fn new(mat: Arc<dyn Material>) -> Self {
    Self { mat }
  }
}

impl Hittable for UnitQuad {
  fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
    let denom = ray.direction.z;
    if denom.abs() < VEC3D_EPSILON {
      return None;
    }
    let t = -ray.origin.z / denom;
    if t < t_min || t > t_max {
      return None;
    }
    let pt = ray.at(t);
    if pt.x < -0.5 || pt.x > 0.5 || pt.y < -0.5 || pt.y > 0.5 {
      return None;
    }
    Some(HitRecord::from_ray(
      ray,
      Direction::new(0.0, 0.0, 1.0),
      t,
      self.mat.clone(),
      UV { u: pt.x + 0.5, v: pt.y + 0.5 },
    ))
  }
  fn bounding_box(&self) -> Aabb {
    Aabb {
      max: Point::new(0.5, 0.5, 0.0),
      min: Point::new(-0.5, -0.5, 0.0),
    }
  }
}
