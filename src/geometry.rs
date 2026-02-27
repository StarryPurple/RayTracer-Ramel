use crate::prelude::*;
use std::sync::Arc;

pub struct HitRecord {
  pub point: Point,
  // always points outside the geometry object (so we do not need a front_face field.)
  // use ray.is_facing(unit_normal) to determine whether this is inside/outside.
  pub unit_normal: Direction,
  pub hit_t: Float,
  pub material: Arc<dyn Material>,
  pub mat_uv: UV,
}

impl HitRecord {
  // require the normal that points outside the geometry object.
  // please send in a unit `normal` vector.
  #[inline]
  pub fn from_ray(
    ray: &Ray,
    unit_normal: Direction,
    hit_t: Float,
    material: Arc<dyn Material>,
    mat_uv: UV
  ) -> Self {
    debug_assert!(
      (unit_normal.length_squared() - 1.0).abs() < const { RAY_EPSILON * RAY_EPSILON },
      "Normal must be normalized! Current length: {}",
      unit_normal.length()
    );
    let point = ray.at(hit_t);
    Self {
      point,
      unit_normal,
      hit_t,
      material,
      mat_uv
    }
  }
}

#[derive(Copy, Clone)]
pub struct Aabb {
  pub min: Point,
  pub max: Point,
}
impl Aabb {
  // pad (so that dx, dy, dz > eps?)

  pub fn union(a: Self, b: Self) -> Self {
    let min = Point {
      x: a.min.x.min(b.min.x),
      y: a.min.y.min(b.min.y),
      z: a.min.z.min(b.min.z),
    };
    let max = Point {
      x: a.max.x.max(b.max.x),
      y: a.max.y.max(b.max.y),
      z: a.max.z.max(b.max.z),
    };
    Self { min, max }
  }
  pub fn longest_axis(&self) -> usize {
    let dx = self.max.x - self.min.x;
    let dy = self.max.y - self.min.y;
    let dz = self.max.z - self.min.z;
    if dx > dy && dx > dz {
      0
    } else if dy > dz {
      1
    } else {
      2
    }
  }
  pub fn might_hit(&self, ray: &Ray, mut t_min: Float, mut t_max: Float) -> bool {
    for i in 0..3 {
      let inv_d = 1.0 / ray.direction[i];
      let mut t0 = (self.min[i] - ray.origin[i]) * inv_d;
      let mut t1 = (self.max[i] - ray.origin[i]) * inv_d;
      if inv_d < 0.0 {
        std::mem::swap(&mut t0, &mut t1);
      }
      t_min = t_min.max(t0);
      t_max = t_max.min(t1);
      if t_max <= t_min {
        return false;
      }
    }
    true
  }
}
impl Default for Aabb {
  fn default() -> Self {
    Self {
      min: Point::new(Float::INFINITY, Float::INFINITY, Float::INFINITY),
      max: Point::new(Float::NEG_INFINITY, Float::NEG_INFINITY, Float::NEG_INFINITY),
    }
  }
}

pub trait Hittable: Send + Sync {
  fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord>;
  fn bounding_box(&self) -> Aabb;
}

mod aggregate;
mod bvh;
mod cone;
mod cube;
mod cylinder;
mod instance;
mod quad;
mod sphere;
mod triangle;

pub use aggregate::Aggregate;
pub use bvh::BvhAggregate;
pub use cube::UnitCube;
pub use instance::Instance;
pub use quad::UnitQuad;
pub use sphere::UnitSphere;
pub use triangle::{TriangleMesh, Triangle};
