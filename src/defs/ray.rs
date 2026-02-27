use crate::config::{Float, RAY_EPSILON};
use crate::defs::vec3d::Vec3d;

pub type Direction = Vec3d;
pub type Point = Vec3d;

// r(t) = ori + dir * t
// dir is not necessarily normalized.
pub struct Ray {
  pub origin: Point,
  pub direction: Direction,
}

impl Ray {
  pub const DISTURBANCE_RATE: Float = RAY_EPSILON;
  #[inline]
  pub fn new(origin: Point, direction: Direction) -> Self {
    Self { origin, direction }
  }
  #[inline]
  pub fn at(&self, t: Float) -> Point {
    self.origin + self.direction * t
  }
  // reflect at point with parameter `hit_t` against the surface with norm `n`.
  // Passing `hit_t` instead of a point, for forcing the section point to be exactly on this ray.
  // Param does not need to concern about the facing of `n` (inside/outside).
  // Add a disturbance to avoid hitting the surface again immediately.
  // r = i - 2(n·i)n
  #[inline]
  pub fn reflect(&self, hit_t: Float, n: Direction) -> Self {
    let n = if self.direction.is_facing(n) { n } else { -n };

    // shadow acne offset, so that the new ray must be on the same side of the incoming one.
    let hit_point = self.at(hit_t) - Self::DISTURBANCE_RATE * n;

    let i = self.direction;
    let direction = i - 2.0 as Float * n.dot(i) * n;

    Self::new(hit_point, direction)
  }
  // refract at point with parameter `hit_t` against the surface with norm `n`.
  // Passing `hit_t` instead of a point, for forcing the section point to be exactly on this ray.
  // Param does not need to concern about the facing of `n` (inside/outside).
  // Add a disturbance to avoid hitting the surface again immediately.
  // when total internal refraction happens, returns `None`.
  #[inline]
  pub fn refract(
    &self,
    hit_t: Float,
    n: Direction,
    ref_idx_in: Float,
    ref_idx_out: Float,
  ) -> Option<Self> {
    let n = if self.direction.is_facing(n) { n } else { -n };

    // shadow acne offset, so that the new ray must be on the opposite side of the incoming one.
    let hit_point = self.at(hit_t) + Self::DISTURBANCE_RATE * n;

    let unit_i = self.direction.normalize();
    let cos_i = unit_i.dot(n).min(1.0);
    let r_perp = (unit_i - n * cos_i) * (ref_idx_in / ref_idx_out);
    let disc = 1.0 as Float - r_perp.length_squared();
    if disc < 0.0 as Float {
      None
    } else {
      let r_para = n * disc.sqrt();
      Some(Self::new(hit_point, r_perp + r_para))
    }
  }
}
