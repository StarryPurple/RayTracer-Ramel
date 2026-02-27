use crate::config::Float;
use crate::defs::ray::{Direction, Point};

#[derive(Clone, Copy, Debug)]
pub struct Mat4d {
  pub data: [Float; 16],
}

impl Mat4d {
  pub const IDENTITY: Mat4d = Mat4d {
    data: [
      1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ],
  };
  pub fn from_translation(dir: Direction) -> Self {
    let mut mat = Self::IDENTITY;
    mat.data[12] = dir.x;
    mat.data[13] = dir.y;
    mat.data[14] = dir.z;
    mat
  }
  pub fn from_scaling(sx: Float, sy: Float, sz: Float) -> Self {
    let mut mat = Self::IDENTITY;
    mat.data[0] = sx;
    mat.data[5] = sy;
    mat.data[10] = sz;
    mat
  }
  pub fn from_rotation_x(angle: Float) -> Self {
    let (s, c) = angle.sin_cos();
    let mut mat = Self::IDENTITY;
    mat.data[5] = c;
    mat.data[6] = s;
    mat.data[9] = -s;
    mat.data[10] = c;
    mat
  }
  pub fn from_rotation_y(angle: Float) -> Self {
    let (s, c) = angle.sin_cos();
    let mut mat = Self::IDENTITY;
    mat.data[0] = c;
    mat.data[2] = -s;
    mat.data[8] = s;
    mat.data[10] = c;
    mat
  }
  pub fn from_rotation_z(angle: Float) -> Self {
    let (s, c) = angle.sin_cos();
    let mut mat = Self::IDENTITY;
    mat.data[0] = c;
    mat.data[1] = s;
    mat.data[4] = -s;
    mat.data[5] = c;
    mat
  }
  pub fn rotation_axis(axis: Direction, angle: Float) -> Self {
    let (s, c) = angle.sin_cos();
    let t = 1.0 - c;
    let v = axis.normalize();
    let mut mat = Self::IDENTITY;

    mat.data[0] = t * v.x * v.x + c;
    mat.data[1] = t * v.x * v.y + v.z * s;
    mat.data[2] = t * v.x * v.z - v.y * s;

    mat.data[4] = t * v.x * v.y - v.z * s;
    mat.data[5] = t * v.y * v.y + c;
    mat.data[6] = t * v.y * v.z + v.x * s;

    mat.data[8] = t * v.x * v.z + v.y * s;
    mat.data[9] = t * v.y * v.z - v.x * s;
    mat.data[10] = t * v.z * v.z + c;

    mat
  }

  pub fn transform_point(&self, pt: Point) -> Point {
    Point::new(
      self.data[0] * pt.x + self.data[4] * pt.y + self.data[8] * pt.z + self.data[12],
      self.data[1] * pt.x + self.data[5] * pt.y + self.data[9] * pt.z + self.data[13],
      self.data[2] * pt.x + self.data[6] * pt.y + self.data[10] * pt.z + self.data[14],
    )
  }
  pub fn transform_vector(&self, vec: Direction) -> Direction {
    Direction::new(
      self.data[0] * vec.x + self.data[4] * vec.y + self.data[8] * vec.z,
      self.data[1] * vec.x + self.data[5] * vec.y + self.data[9] * vec.z,
      self.data[2] * vec.x + self.data[6] * vec.y + self.data[10] * vec.z,
    )
  }
  pub fn transpose_transform_point(&self, pt: Point) -> Point {
    Point::new(
      self.data[0] * pt.x + self.data[1] * pt.y + self.data[2] * pt.z + self.data[3],
      self.data[4] * pt.x + self.data[5] * pt.y + self.data[6] * pt.z + self.data[7],
      self.data[8] * pt.x + self.data[9] * pt.y + self.data[10] * pt.z + self.data[11],
    )
  }
  pub fn transpose_transform_vector(&self, vec: Direction) -> Point {
    Direction::new(
      self.data[0] * vec.x + self.data[1] * vec.y + self.data[2] * vec.z,
      self.data[4] * vec.x + self.data[5] * vec.y + self.data[6] * vec.z,
      self.data[8] * vec.x + self.data[9] * vec.y + self.data[10] * vec.z,
    )
  }
  pub fn transpose(&self) -> Self {
    let mut data = [0.0; 16];
    for i in 0..4 {
      for j in 0..4 {
        data[i * 4 + j] = self.data[j * 4 + i];
      }
    }
    Self { data }
  }
  pub fn inverse(&self) -> Option<Self> {
    let mut aug = [[0.0 as Float; 8]; 4];
    let identity = Self::IDENTITY;

    for i in 0..4 {
      for j in 0..4 {
        aug[i][j] = self.data[j * 4 + i];
        aug[i][j + 4] = identity.data[j * 4 + i];
      }
    }

    for i in 0..4 {
      let mut pivot = i;
      for j in (i + 1)..4 {
        if aug[j][i].abs() > aug[pivot][i].abs() {
          pivot = j;
        }
      }

      aug.swap(i, pivot);

      if aug[i][i].abs() < 1e-12 {
        return None;
      }

      let divisor = aug[i][i];
      for j in i..8 {
        aug[i][j] /= divisor;
      }

      for j in 0..4 {
        if i != j {
          let factor = aug[j][i];
          for k in i..8 {
            aug[j][k] -= factor * aug[i][k];
          }
        }
      }
    }

    let mut data = [0.0; 16];
    for i in 0..4 {
      for j in 0..4 {
        data[j * 4 + i] = aug[i][j + 4];
      }
    }

    Some(Self { data })
  }
}

impl std::ops::Add for Mat4d {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    let mut data = [0.0; 16];
    for i in 0..16 {
      data[i] = self.data[i] + rhs.data[i];
    }
    Self { data }
  }
}

impl std::ops::Sub for Mat4d {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    let mut data = [0.0; 16];
    for i in 0..16 {
      data[i] = self.data[i] - rhs.data[i];
    }
    Self { data }
  }
}

impl std::ops::Mul<Float> for Mat4d {
  type Output = Self;
  fn mul(self, rhs: Float) -> Self {
    let mut data = [0.0; 16];
    for i in 0..16 {
      data[i] = self.data[i] * rhs;
    }
    Self { data }
  }
}

impl std::ops::Mul<Mat4d> for Mat4d {
  type Output = Self;
  fn mul(self, rhs: Mat4d) -> Self {
    let mut data = [0.0; 16];
    for row in 0..4 {
      for col in 0..4 {
        let mut sum = 0.0;
        for k in 0..4 {
          sum += self.data[k * 4 + row] * rhs.data[col * 4 + k];
        }
        data[col * 4 + row] = sum;
      }
    }
    Self { data }
  }
}
