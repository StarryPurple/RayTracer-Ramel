use crate::geometry::Aabb;
use crate::prelude::*;

/// UnitSphere has center (0, 0, 0) and radius 1.
pub struct UnitSphere {
  mat: Arc<dyn Material>,
}

impl UnitSphere {
  pub fn new(mat: Arc<dyn Material>) -> Self {
    Self { mat }
  }
  pub fn new_arc(mat: Arc<dyn Material>) -> Arc<Self> {
    Arc::new(Self::new(mat))
  }
}

impl Hittable for UnitSphere {
  fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
    let oc = ray.origin;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.length_squared() - 1.0;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
      return None;
    }
    let sqrtd = discriminant.sqrt();
    let mut root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
      root = (-half_b + sqrtd) / a;
      if root < t_min || t_max < root {
        return None;
      }
    }

    let point = ray.at(root);
    let normal = point.normalize(); // precision not enough...

    let theta = (-point.y).acos();
    let phi = (-point.z).atan2(point.x) + PI;

    let u = phi / (2.0 * PI);
    let v = theta / PI;

    Some(HitRecord::from_ray(
      ray,
      normal,
      root,
      self.mat.clone(),
      UV { u, v }
    ))
  }
  fn bounding_box(&self) -> Aabb {
    Aabb { max: Point::new(1.0, 1.0, 1.0), min: Point::new(-1.0, -1.0, -1.0) }
  }
}
