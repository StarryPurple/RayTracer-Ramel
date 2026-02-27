use crate::prelude::*;

struct Perlin {
  rand_vec: Vec<Vec3d>,
  perm_x: Vec<i32>,
  perm_y: Vec<i32>,
  perm_z: Vec<i32>,
}

impl Perlin {
  pub fn new(mut seed: u32) -> Self {
    let mut rand_vec = Vec::with_capacity(256);
    for _ in 0..256 {
      rand_vec.push(Vec3d::random_unit());
    }

    Self {
      rand_vec,
      perm_x: Self::generate_perm(&mut seed),
      perm_y: Self::generate_perm(&mut seed),
      perm_z: Self::generate_perm(&mut seed),
    }
  }

  pub fn noise(&self, p: &Point) -> Float {
    let u = p.x - p.x.floor();
    let v = p.y - p.y.floor();
    let w = p.z - p.z.floor();

    // 厄米特平滑 (Hermite Smoothing)
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let i = p.x.floor() as i32;
    let j = p.y.floor() as i32;
    let k = p.z.floor() as i32;

    let mut accum = 0.0;
    for di in 0..2 {
      for dj in 0..2 {
        for dk in 0..2 {
          let vec = self.rand_vec[(
            self.perm_x[((i + di) & 255) as usize] ^
              self.perm_y[((j + dj) & 255) as usize] ^
              self.perm_z[((k + dk) & 255) as usize]
          ) as usize];

          let weight_v = Vec3d::new(u - di as Float, v - dj as Float, w - dk as Float);
          accum += (di as Float * uu + (1.0 - di as Float) * (1.0 - uu)) *
            (dj as Float * vv + (1.0 - dj as Float) * (1.0 - vv)) *
            (dk as Float * ww + (1.0 - dk as Float) * (1.0 - ww)) *
            vec.dot(weight_v);
        }
      }
    }
    accum
  }

  fn generate_perm(seed: &mut u32) -> Vec<i32> {
    let mut p: Vec<i32> = (0..256).collect();
    for i in (1..256).rev() {
      *seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
      let target = (*seed as usize) % (i as usize + 1);
      p.swap(i as usize, target);
    }
    /*
    let mut rng = rand::rng();
    for i in (1..256).rev() {
      let target = rng.random_range(0..=i);
      p.swap(i as usize, target as usize);
    } */
    p
  }
}

pub struct PerlinNoiseTexture {
  noise: Perlin,
  scale: Float,
}

impl PerlinNoiseTexture {
  pub fn new(scale: Float, seed: u32) -> Self {
    Self { noise: Perlin::new(seed), scale }
  }
}

impl Texture for PerlinNoiseTexture {
  fn value(&self, _uv: UV, p: &Point) -> ColorRgb {
    ColorRgb::WHITE * 0.5 * (1.0 + self.noise.noise(&(self.scale * *p)))
  }
}