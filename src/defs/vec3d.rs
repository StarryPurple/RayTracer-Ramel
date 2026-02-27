use crate::config::{Float, RAY_EPSILON, VEC3D_EPSILON};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3d {
  pub x: Float,
  pub y: Float,
  pub z: Float,
}

impl Vec3d {
  pub const LENGTH_EPSILON: Float = RAY_EPSILON;
  pub const NEAR_ZERO_SQ: Float = VEC3D_EPSILON * VEC3D_EPSILON;
  pub const ZERO: Vec3d = Vec3d { x: 0.0, y: 0.0, z: 0.0 };

  #[inline]
  pub fn new(x: Float, y: Float, z: Float) -> Self {
    Self { x, y, z }
  }
  #[inline]
  pub fn dot(&self, rhs: Self) -> Float {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
  }
  #[inline]
  pub fn cross(&self, rhs: Self) -> Self {
    Self {
      x: self.y * rhs.z - self.z * rhs.y,
      y: self.z * rhs.x - self.x * rhs.z,
      z: self.x * rhs.y - self.y * rhs.x,
    }
  }

  #[inline]
  pub fn length_squared(&self) -> Float {
    self.dot(*self)
  }
  #[inline]
  pub fn length(&self) -> Float {
    self.length_squared().sqrt()
  }
  #[inline]
  pub fn normalize(&self) -> Self {
    let l2 = self.length_squared();
    if l2 < Self::NEAR_ZERO_SQ {
      Self::ZERO
    } else {
      let inv_len = 1.0 / l2.sqrt();
      *self * inv_len
    }
  }

  // whether the two vectors (as directions) are facing a same semisphere.
  #[inline]
  pub fn is_facing(&self, normal: Self) -> bool {
    self.dot(normal) > 0.0
  }
  #[inline]
  pub fn near_zero(&self) -> bool {
    self.length_squared() < Self::NEAR_ZERO_SQ
  }

  #[inline]
  pub fn random_unit() -> Self {
    let mut rng = rand::rng();
    let theta: Float = rng.random_range(0.0..std::f64::consts::TAU) as Float;
    let z: Float = rng.random_range(-1.0..1.0);
    let r = (1.0 - z * z).max(0.0).sqrt();
    let (sin_t, cos_t) = theta.sin_cos();
    Self::new(r * cos_t, r * sin_t, z)
  }
}

impl std::ops::Add for Vec3d {
  type Output = Self;
  #[inline]
  fn add(self, rhs: Self) -> Self {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl std::ops::AddAssign for Vec3d {
  #[inline]
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
    self.z += rhs.z;
  }
}

impl std::ops::Sub for Vec3d {
  type Output = Self;
  #[inline]
  fn sub(self, rhs: Self) -> Self {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl std::ops::SubAssign for Vec3d {
  #[inline]
  fn sub_assign(&mut self, rhs: Self) {
    self.x -= rhs.x;
    self.y -= rhs.y;
    self.z -= rhs.z;
  }
}

impl std::ops::Mul<Float> for Vec3d {
  type Output = Self;
  #[inline]
  fn mul(self, rhs: Float) -> Self {
    Self {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}

impl std::ops::Mul<Vec3d> for Float {
  type Output = Vec3d;
  #[inline]
  fn mul(self, rhs: Vec3d) -> Vec3d {
    rhs * self
  }
}

impl std::ops::Div<Float> for Vec3d {
  type Output = Self;
  #[inline]
  fn div(self, rhs: Float) -> Self {
    #[cfg(debug_assertions)]
    if rhs.abs() < VEC3D_EPSILON {
      eprintln!("Warning: Vec3d division by zero.");
      // just let this blow up and return something like [inf, inf, inf].
    }
    let inv = 1.0 / rhs;
    self * inv
  }
}

impl std::ops::Neg for Vec3d {
  type Output = Self;
  #[inline]
  fn neg(self) -> Self {
    Self { x: -self.x, y: -self.y, z: -self.z }
  }
}

impl std::ops::Index<usize> for Vec3d {
  type Output = Float;
  #[inline]
  fn index(&self, index: usize) -> &Self::Output {
    match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => panic!("Index out of bounds for Vec3d"),
    }
  }
}

impl std::ops::IndexMut<usize> for Vec3d {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    match index {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      _ => panic!("Index out of bounds for Vec3d"),
    }
  }
}

impl Default for Vec3d {
  fn default() -> Self {
    Self::ZERO
  }
}