use crate::color::C;

/// A trait that represents blendable types.
/// Implementing this trait enables blending
/// that complies with the W3C specification.
pub trait Blend: Sized {
  fn from_color(c: C) -> Self;
  fn to_color(&self) -> C;

  fn normal(&self, backdrop: &impl Blend) -> Self {
    Self::composite_separable(|_cb, cs| cs)(backdrop, self)
  }

  /// https://drafts.csswg.org/compositing-1/#blendingseparable
  fn composite_separable<B: Blend, F>(f: F) -> impl Fn(&B, &Self) -> Self
  where
    F: Fn(f32, f32) -> f32,
  {
    move |backdrop, src| -> Self {
      let cb = backdrop.to_color();
      let cs = src.to_color();

      if cs.a == 0. && cb.a == 0. {
        return Self::from_color(C::ZERO);
      }

      // Blending: Cr = (1 - αb) x Cs + αb x B(Cb, Cs)
      // https://drafts.csswg.org/compositing-1/#blending
      let blended_r = (1. - cb.a) * cs.r + cb.a * f(cb.r, cs.r);
      let blended_g = (1. - cb.a) * cs.g + cb.a * f(cb.g, cs.g);
      let blended_b = (1. - cb.a) * cs.b + cb.a * f(cb.b, cs.b);

      // Composite: Co = αs x Fa x Cs + αb x Fb x Cb
      // https://drafts.csswg.org/compositing-1/#porterduffcompositingoperators
      let pm_r = cs.a * 1. * blended_r + cb.a * (1. - cs.a) * cb.r;
      let pm_g = cs.a * 1. * blended_g + cb.a * (1. - cs.a) * cb.g;
      let pm_b = cs.a * 1. * blended_b + cb.a * (1. - cs.a) * cb.b;
      // αo = αs x Fa + αb x Fb
      let a0 = cs.a * 1. + cb.a * (1. - cs.a);

      if a0 == 0. {
        return Self::from_color(C::ZERO);
      }

      let r = (pm_r / a0).clamp(0.0, 1.0);
      let g = (pm_g / a0).clamp(0.0, 1.0);
      let b = (pm_b / a0).clamp(0.0, 1.0);

      Self::from_color(C::new(r, g, b, a0))
    }
  }

  /// https://drafts.csswg.org/compositing-1/#blendingnonseparable
  fn composite_non_separable<B: Blend, F>(f: F) -> impl Fn(&B, &Self) -> Self
  where
    F: Fn((f32, f32, f32), (f32, f32, f32)) -> (f32, f32, f32),
  {
    move |backdrop, src| -> Self {
      let cb = backdrop.to_color();
      let cs = src.to_color();

      if cs.a == 0. && cb.a == 0. {
        return Self::from_color(C::ZERO);
      }

      // Blending: Cr = (1 - αb) x Cs + αb x B(Cb, Cs)
      // https://drafts.csswg.org/compositing-1/#blending
      let (b_r, b_g, b_b) = f((cb.r, cb.g, cb.b), (cs.r, cs.g, cs.b));
      let blended_r = (1. - cb.a) * cs.r + cb.a * b_r;
      let blended_g = (1. - cb.a) * cs.g + cb.a * b_g;
      let blended_b = (1. - cb.a) * cs.b + cb.a * b_b;

      // Composite: Co = αs x Fa x Cs + αb x Fb x Cb
      // https://drafts.csswg.org/compositing-1/#porterduffcompositingoperators
      let pm_r = cs.a * 1. * blended_r + cb.a * (1. - cs.a) * cb.r;
      let pm_g = cs.a * 1. * blended_g + cb.a * (1. - cs.a) * cb.g;
      let pm_b = cs.a * 1. * blended_b + cb.a * (1. - cs.a) * cb.b;
      // αo = αs x Fa + αb x Fb
      let a0 = cs.a * 1. + cb.a * (1. - cs.a);

      if a0 == 0. {
        return Self::from_color(C::ZERO);
      }

      let r = (pm_r / a0).clamp(0.0, 1.0);
      let g = (pm_g / a0).clamp(0.0, 1.0);
      let b = (pm_b / a0).clamp(0.0, 1.0);

      Self::from_color(C::new(r, g, b, a0))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug)]
  pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
  }

  impl Rgba {
    fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
      Self { r, g, b, a }
    }
  }

  impl Blend for Rgba {
    fn from_color(c: C) -> Self {
      Rgba {
        r: c.r,
        g: c.g,
        b: c.b,
        a: c.a,
      }
    }

    fn to_color(&self) -> C {
      C::new(self.r, self.g, self.b, self.a)
    }
  }

  fn assert_rgba(actual: Rgba, expected: Rgba) {
    let eps = 1e-4;

    assert!(
      (actual.r - expected.r).abs() < eps
        && (actual.g - expected.g).abs() < eps
        && (actual.b - expected.b).abs() < eps
        && (actual.a - expected.a).abs() < eps,
      "expected: {:?}, actual: {:?}",
      expected,
      actual
    );
  }

  #[test]
  fn test_transparent_over_transparent() {
    let bg = Rgba::new(1.0, 0.0, 0.0, 0.0);
    let fg = Rgba::new(0.0, 0.0, 1.0, 0.0);

    assert_rgba(fg.normal(&bg), Rgba::new(0.0, 0.0, 0.0, 0.0));
  }

  #[test]
  fn test_normal_opaque_over_opaque() {
    let bg = Rgba::new(1.0, 0.0, 0.0, 1.0);
    let fg = Rgba::new(0.0, 0.0, 1.0, 1.0);

    assert_rgba(fg.normal(&bg), Rgba::new(0.0, 0.0, 1.0, 1.0));
  }

  #[test]
  fn test_normal_transparent_over_opaque() {
    let bg = Rgba::new(1.0, 0.0, 0.0, 1.0);
    let fg = Rgba::new(0.0, 0.0, 1.0, 0.0);

    assert_rgba(fg.normal(&bg), Rgba::new(1.0, 0.0, 0.0, 1.0));
  }

  #[test]
  fn test_normal_semi_transparent_over_opaque() {
    let bg = Rgba::new(1.0, 0.0, 0.0, 1.0);
    let fg = Rgba::new(0.0, 0.0, 1.0, 0.5);

    assert_rgba(fg.normal(&bg), Rgba::new(0.5, 0.0, 0.5, 1.0));
  }

  #[test]
  fn test_normal_semi_transparent_over_semi_transparent() {
    let bg = Rgba::new(1.0, 0.0, 0.0, 0.5);
    let fg = Rgba::new(0.0, 1.0, 0.0, 0.5);

    assert_rgba(fg.normal(&bg), Rgba::new(0.33333334, 0.6666667, 0.0, 0.75));
  }
}
