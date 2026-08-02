use crate::{
  color::C,
  porter_duff::{CompositeOperator, PorterDuff},
};

/// A trait that represents blendable types.
/// Implementing this trait enables blending
/// that complies with the W3C specification.
pub trait Blend: Sized {
  fn from_color(c: C) -> Self;
  fn to_color(&self) -> C;

  /// The utility function for `normal` blend. It is composited using `SourceOver`.
  /// Use `normal_with`, if you want to blend specifying the Porter-Duff composite operator.
  fn normal(&self, backdrop: &impl Blend) -> Self {
    Self::composite_separable(|_cb, cs| cs, PorterDuff::SourceOver)(backdrop, self)
  }

  /// `normal` blend function that allows color blending using the Porter-Duff composite operator.
  fn normal_with(&self, backdrop: &impl Blend, op: PorterDuff) -> Self {
    Self::composite_separable(|_cb, cs| cs, op)(backdrop, self)
  }

  /// General composite function for each component of the result color is completely determined by
  /// the corresponding components of the constituent backdrop and source colors.
  /// https://drafts.csswg.org/compositing-1/#blendingseparable
  fn composite_separable<B: Blend, F, Op: CompositeOperator>(
    f: F,
    op: Op,
  ) -> impl Fn(&B, &Self) -> Self
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
      let (fa, fb) = op.fractions(cs.a, cb.a);
      let pm_r = cs.a * fa * blended_r + cb.a * fb * cb.r;
      let pm_g = cs.a * fa * blended_g + cb.a * fb * cb.g;
      let pm_b = cs.a * fa * blended_b + cb.a * fb * cb.b;
      // αo = αs x Fa + αb x Fb
      let a0 = cs.a * fa + cb.a * fb;

      if a0 == 0. {
        return Self::from_color(C::ZERO);
      }

      let r = (pm_r / a0).clamp(0.0, 1.0);
      let g = (pm_g / a0).clamp(0.0, 1.0);
      let b = (pm_b / a0).clamp(0.0, 1.0);

      Self::from_color(C::new(r, g, b, a0))
    }
  }

  /// General composite function for all color components in combination.
  /// https://drafts.csswg.org/compositing-1/#blendingnonseparable
  fn composite_non_separable<B: Blend, F, Op: CompositeOperator>(
    f: F,
    op: Op,
  ) -> impl Fn(&B, &Self) -> Self
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
      let (fa, fb) = op.fractions(cs.a, cb.a);
      let pm_r = cs.a * fa * blended_r + cb.a * fb * cb.r;
      let pm_g = cs.a * fa * blended_g + cb.a * fb * cb.g;
      let pm_b = cs.a * fa * blended_b + cb.a * fb * cb.b;
      // αo = αs x Fa + αb x Fb
      let a0 = cs.a * fa + cb.a * fb;

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
