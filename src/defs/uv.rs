use std::ops::{Add, Index, IndexMut, Mul};
use crate::config::Float;

#[derive(Copy, Clone, Debug, Default)]
pub struct UV {
  pub u: Float,
  pub v: Float,
}

impl UV {
  pub fn new(u: Float, v: Float) -> Self {
    UV { u, v }
  }
}

impl Index<usize> for UV {
  type Output = Float;
  fn index(&self, index: usize) -> &Self::Output {
    match index { 
      0 => &self.u,
      1 => &self.v,
      _ => panic!("index out of bounds"),
    }
  }
}

impl IndexMut<usize> for UV {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    match index { 
      0 => &mut self.u,
      1 => &mut self.v,
      _ => panic!("index out of bounds"),
    }
  }
}

impl Add for UV {
  type Output = UV;
  fn add(self, other: UV) -> UV {
    UV { u: self.u + other.u, v: self.v + other.v }
  }
}

impl Mul<Float> for UV {
  type Output = UV;
  fn mul(self, other: Float) -> UV {
    UV { u: self.u * other, v: self.v * other }
  }
}

impl Mul<UV> for Float {
  type Output = UV;
  fn mul(self, other: UV) -> UV {
    UV { u: other.u * self, v: other.v * self }
  }
}