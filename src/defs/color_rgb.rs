use crate::config::{COLOR_EPSILON, Float};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorRgb {
  pub r: Float,
  pub g: Float,
  pub b: Float,
}

impl ColorRgb {
  pub const NEAR_ZERO: Float = COLOR_EPSILON;

  pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0 };
  pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0 };
  pub const SKYBLUE: Self = Self { r: 0.5, g: 0.7, b: 1.0 };
  pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0 };
  pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0 };
  pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0 };
  pub const MAGENTA: Self = Self { r: 1.0, g: 0.0, b: 1.0 };
  pub const YELLOW: Self = Self { r: 1.0, g: 1.0, b: 0.0 };
  pub const CYAN: Self = Self { r: 0.0, g: 1.0, b: 1.0 };

  #[inline]
  pub fn new(r: Float, g: Float, b: Float) -> Self {
    Self { r, g, b }
  }

  #[inline]
  pub fn into_rgba(self) -> image::Rgba<u8> {
    if !Self::check_valid(self.r, self.g, self.b) {
      return image::Rgba([255, 0, 255, 255]); // magenta color as a signal
    }
    let to_u8 = |x: Float| (x.clamp(0.0, 1.0) * 255.999 as Float) as u8;
    image::Rgba([to_u8(self.r), to_u8(self.g), to_u8(self.b), 255])
  }

  #[inline]
  pub fn from_rgba(pixel: image::Rgba<u8>) -> Self {
    let r = pixel[0] as Float / 255.0;
    let g = pixel[1] as Float / 255.0;
    let b = pixel[2] as Float / 255.0;
    Self::new(r, g, b)
  }

  fn check_valid(r: Float, g: Float, b: Float) -> bool {
    static WARN_ONCE: std::sync::Once = std::sync::Once::new();

    let is_valid = (0.0..=1.0).contains(&r) && (0.0..=1.0).contains(&g) && (0.0..=1.0).contains(&b);

    if !is_valid {
      WARN_ONCE.call_once(|| {
        eprintln!("Warning: Some pixels are out of range and replaced with Magenta.");
      });

      #[cfg(debug_assertions)]
      eprintln!("Shader output out of range! Got: (r: {r}, g: {g}, b: {b})");
    }

    is_valid
  }

  #[inline]
  pub fn to_linear(self) -> Self {
    Self::new(self.r.powf(2.2), self.g.powf(2.2), self.b.powf(2.2))
  }

  #[inline]
  pub fn to_gamma(self) -> Self {
    const GAMMA_INV: Float = 1.0 / 2.2;
    Self::new(
      self.r.powf(GAMMA_INV),
      self.g.powf(GAMMA_INV),
      self.b.powf(GAMMA_INV),
    )
  }

  #[inline]
  /// 0 -> color1, 1 -> color2
  pub fn lerp(color1: Self, color2: Self, t: Float) -> Self {
    color1 * (1.0 - t) + color2 * t
  }
  pub fn mix_samples(samples: &[(ColorRgb, Float)]) -> Self {
    let mut tot = 0.0;
    let mut res = ColorRgb::BLACK;

    for (_, weight) in samples {
      tot += *weight;
    }

    if tot <= Self::NEAR_ZERO {
      return ColorRgb::BLACK;
    }

    for (color, weight) in samples {
      res += *color * (*weight / tot);
    }

    res
  }
}

impl std::ops::Add for ColorRgb {
  type Output = Self;
  #[inline]
  fn add(self, rhs: Self) -> Self {
    Self {
      r: self.r + rhs.r,
      g: self.g + rhs.g,
      b: self.b + rhs.b,
    }
  }
}

impl std::ops::AddAssign for ColorRgb {
  #[inline]
  fn add_assign(&mut self, rhs: Self) {
    self.r += rhs.r;
    self.g += rhs.g;
    self.b += rhs.b;
  }
}

impl std::ops::Mul for ColorRgb {
  type Output = Self;
  #[inline]
  fn mul(self, rhs: Self) -> Self {
    Self {
      r: self.r * rhs.r,
      g: self.g * rhs.g,
      b: self.b * rhs.b,
    }
  }
}

impl std::ops::Mul<Float> for ColorRgb {
  type Output = Self;
  #[inline]
  fn mul(self, rhs: Float) -> Self {
    Self {
      r: self.r * rhs,
      g: self.g * rhs,
      b: self.b * rhs,
    }
  }
}

impl std::ops::MulAssign<Float> for ColorRgb {
  #[inline]
  fn mul_assign(&mut self, rhs: Float) {
    self.r *= rhs;
    self.g *= rhs;
    self.b *= rhs;
  }
}

impl std::ops::Div<Float> for ColorRgb {
  type Output = Self;
  #[inline]
  fn div(self, rhs: Float) -> Self {
    if rhs.abs() < Self::NEAR_ZERO {
      #[cfg(debug_assertions)]
      {
        static DIV_ZERO_WARN: std::sync::Once = std::sync::Once::new();
        DIV_ZERO_WARN.call_once(|| {
          eprintln!("Warning: Division by zero (or near-zero) detected in ColorRgb.");
        });
      }
      return Self::BLACK;
    }
    let inv = 1.0 / rhs;
    self * inv
  }
}

impl std::ops::DivAssign<Float> for ColorRgb {
  #[inline]
  fn div_assign(&mut self, rhs: Float) {
    if rhs.abs() < Self::NEAR_ZERO {
      #[cfg(debug_assertions)]
      {
        static DIV_ZERO_WARN: std::sync::Once = std::sync::Once::new();
        DIV_ZERO_WARN.call_once(|| {
          eprintln!("Warning: Division assignment by zero (or near-zero) detected in ColorRgb.");
        });
      }
    }
    let inv = 1.0 / rhs;
    self.r *= inv;
    self.g *= inv;
    self.b *= inv;
  }
}
