use crate::prelude::*;

/// [-0.5, 0.5]^3.
/// material order: +X(Right), -X, +Y(Up), -Y, +Z(Front), -Z.
/// material direction is fixed:
/// FRBL: Vup = +Y, U: Vup = -Z, D: Vup = +Z
pub struct UnitCube {
  pub mat: [Arc<dyn Material>; 6],
}

impl UnitCube {
  /// material order: +X(Right), -X, +Y(Up), -Y, +Z(Front), -Z.
  /// the directions of the materials shall be handled by caller.
  pub fn new(mat: [Arc<dyn Material>; 6]) -> Self {
    Self { mat }
  }
  pub fn from_one(mat: Arc<dyn Material>) -> Self {
    Self { mat: [mat.clone(), mat.clone(), mat.clone(), mat.clone(), mat.clone(), mat.clone()] }
  }
}

impl Hittable for UnitCube {
  fn hit(&self, ray: &Ray, t_min: Float, t_max: Float) -> Option<HitRecord> {
    let mut point: Point = Point::default();
    let mut hit_t: Float = t_max;
    let mut u_raw: Float = 0.0;
    let mut v_raw: Float = 0.0;
    let mut normal: Direction = Direction::default();
    let mut face_order: Option<usize> = None;
    for axis in 0..3 {
      let dir_inv = 1.0 / ray.direction[axis];
      for is_pos in [true, false] {
        let t = (if is_pos { 0.5 } else { -0.5 } - ray.origin[axis]) * dir_inv;
        if face_order.is_some() && t > hit_t {
          continue;
        }
        if t < t_min || t_max < t {
          continue;
        }
        point = ray.at(t);
        let (u, v) = match (axis, is_pos) {
          (0, true)  => (-point.z, point.y), // +X (Right)
          (0, false) => (point.z, point.y),  // -X (Left)
          (1, true)  => (point.x, -point.z), // +Y (Up)
          (1, false) => (point.x, point.z),  // -Y (Down)
          (2, true)  => (point.x, point.y),  // +Z (Front)
          (2, false) => (-point.x, point.y), // -Z (Back)
          _ => unreachable!()
        };
        const BOUND: Float = 0.5 + VEC3D_EPSILON;
        if u < -BOUND || u > BOUND || v < -BOUND || v > BOUND {
          continue;
        }
        (u_raw, v_raw) = (u, v);
        hit_t = t;
        face_order = Some(axis * 2 + if is_pos { 0 } else { 1 });
        normal = Direction::ZERO;
        normal[axis] = if is_pos { 1.0 } else { -1.0 };
      }
    }
    face_order.map(|order| HitRecord {
      point,
      hit_t,
      unit_normal: normal,
      material: self.mat[order].clone(),
      mat_uv: UV { u: u_raw + 0.5, v: v_raw + 0.5 },
    })
  }
  fn bounding_box(&self) -> Aabb {
    Aabb { max: Point::new(0.5, 0.5, 0.5), min: Point::new(-0.5, -0.5, -0.5) }
  }
}
