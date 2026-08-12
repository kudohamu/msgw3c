/// Represents simple color type of RGBA format.
/// NOTE: RGB channels MUST be `premultiplied` values.
/// Each value ranges from 0 to 1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct C {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl C {
  pub const TRANSPARENT: Self = Self {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
  };

  #[inline]
  pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
    C { r, g, b, a }
  }

  #[inline]
  pub fn from_straight_alpha(r: f32, g: f32, b: f32, a: f32) -> Self {
    Self::new(r * a, g * a, b * a, a)
  }

  #[inline]
  pub fn to_straight_alpha(&self) -> Self {
    if self.a == 0. {
      return C::TRANSPARENT;
    }

    let inv_a = 1. / self.a;
    let r = (self.r * inv_a).clamp(0., 1.);
    let g = (self.g * inv_a).clamp(0., 1.);
    let b = (self.b * inv_a).clamp(0., 1.);

    C::new(r, g, b, self.a)
  }
}
