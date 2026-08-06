/// Represents simple color type of RGBA format.
/// NOTE: RGB values MUST be `non-premultiplied` values.
/// Each value ranges from 0 to 1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct C {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl C {
  pub const ZERO: Self = Self {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
  };

  pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
    C { r, g, b, a }
  }
}
