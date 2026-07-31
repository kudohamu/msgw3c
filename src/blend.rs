/// A trait that represents blendable types.
/// Implementing this trait enables blending
/// that complies with the W3C specification.
pub trait Blend: Sized {
  fn non_premultiplied_rgba(&self) -> (f32, f32, f32, f32);

  fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self;

  fn normal(&self, backdrop: &impl Blend) -> Self {
    Self::composite_separable(|_cb, cs| cs)(backdrop, self)
  }

  fn composite_separable<B: Blend, F>(f: F) -> impl Fn(&B, &Self) -> Self
  where
    F: Fn(f32, f32) -> f32,
  {
    move |backdrop, src| -> Self {
      let (back_r, back_g, back_b, back_a) = backdrop.non_premultiplied_rgba();
      let (src_r, src_g, src_b, src_a) = src.non_premultiplied_rgba();

      if src_a == 0. && back_a == 0. {
        return Self::from_rgba(0., 0., 0., 0.);
      }

      // Blending: Cr = (1 - αb) x Cs + αb x B(Cb, Cs)
      // https://drafts.csswg.org/compositing-1/#blending
      let blended_r = (1. - back_a) * src_r + back_a * f(back_r, src_r);
      let blended_g = (1. - back_a) * src_g + back_a * f(back_g, src_g);
      let blended_b = (1. - back_a) * src_b + back_a * f(back_b, src_b);

      // Composite: Co = αs x Fa x Cs + αb x Fb x Cb
      // https://drafts.csswg.org/compositing-1/#porterduffcompositingoperators
      let pm_r = src_a * 1. * blended_r + back_a * (1. - src_a) * back_r;
      let pm_g = src_a * 1. * blended_g + back_a * (1. - src_a) * back_g;
      let pm_b = src_a * 1. * blended_b + back_a * (1. - src_a) * back_b;
      // αo = αs x Fa + αb x Fb
      let a0 = src_a * 1. + back_a * (1. - src_a);

      if a0 == 0. {
        return Self::from_rgba(0., 0., 0., 0.);
      }

      let r = (pm_r / a0).clamp(0.0, 1.0);
      let g = (pm_g / a0).clamp(0.0, 1.0);
      let b = (pm_b / a0).clamp(0.0, 1.0);

      Self::from_rgba(r, g, b, a0)
    }
  }

  fn composite_non_separable<B: Blend, F>(f: F) -> impl Fn(&B, &Self) -> Self
  where
    F: Fn((f32, f32, f32), (f32, f32, f32)) -> (f32, f32, f32),
  {
    |backdrop, src| -> Self { Self::from_rgba(0., 0., 0., 1.) }
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

  impl Blend for Rgba {
    fn non_premultiplied_rgba(&self) -> (f32, f32, f32, f32) {
      (self.r, self.g, self.b, self.a)
    }

    fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
      Self { r, g, b, a }
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
    let bg = Rgba::from_rgba(1.0, 0.0, 0.0, 0.0);
    let fg = Rgba::from_rgba(0.0, 0.0, 1.0, 0.0);

    assert_rgba(fg.normal(&bg), Rgba::from_rgba(0.0, 0.0, 0.0, 0.0));
  }

  #[test]
  fn test_normal_opaque_over_opaque() {
    let bg = Rgba::from_rgba(1.0, 0.0, 0.0, 1.0);
    let fg = Rgba::from_rgba(0.0, 0.0, 1.0, 1.0);

    assert_rgba(fg.normal(&bg), Rgba::from_rgba(0.0, 0.0, 1.0, 1.0));
  }

  #[test]
  fn test_normal_transparent_over_opaque() {
    let bg = Rgba::from_rgba(1.0, 0.0, 0.0, 1.0);
    let fg = Rgba::from_rgba(0.0, 0.0, 1.0, 0.0);

    assert_rgba(fg.normal(&bg), Rgba::from_rgba(1.0, 0.0, 0.0, 1.0));
  }

  #[test]
  fn test_normal_semi_transparent_over_opaque() {
    let bg = Rgba::from_rgba(1.0, 0.0, 0.0, 1.0);
    let fg = Rgba::from_rgba(0.0, 0.0, 1.0, 0.5);

    assert_rgba(fg.normal(&bg), Rgba::from_rgba(0.5, 0.0, 0.5, 1.0));
  }

  #[test]
  fn test_normal_semi_transparent_over_semi_transparent() {
    let bg = Rgba::from_rgba(1.0, 0.0, 0.0, 0.5);
    let fg = Rgba::from_rgba(0.0, 1.0, 0.0, 0.5);

    assert_rgba(
      fg.normal(&bg),
      Rgba::from_rgba(0.33333334, 0.6666667, 0.0, 0.75),
    );
  }
}
